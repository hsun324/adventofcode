#include <chrono>
#include <cstdio>
#include <iomanip>
#include <iostream>
#include <thread>
#include <set>
#include <vector>

/* A range covering [l, r]. */
template<typename Int>
struct range {
    Int l, r;

    range(Int l = 0, Int r = 0) : l(l), r(std::max(l, r)) { }
    range(const range&) = default;

    bool contains(Int c) const { return l <= c && c < r; }
    Int size() const { return r - l; }

    operator bool() const { return l < r; }
};

template<typename AInt, typename BInt>
auto operator&(const range<AInt> &a, const range<BInt> &b) -> range<decltype(AInt() + BInt())> {
    auto l = std::max(a.l, b.l);
    auto r = std::min(a.r, b.r);
    if (l <= r)
        return {l, r};
    return {};
}

/* Define a vector using partial ordering. */
template<typename Int>
struct vec3 {
    Int x, y, z;

    vec3(Int x, Int y, Int z) : x(x), y(y), z(z) { }
    vec3(const vec3&) = default;

    template<Int>
    friend std::ostream &operator<<(std::ostream&, const vec3<Int>&);
};

template<typename AInt, typename BInt>
auto operator+(const vec3<AInt> &a, const vec3<BInt> &b) -> vec3<decltype(AInt() + BInt())> {
    return {a.x + b.x, a.y + b.y, a.z + b.z};
}

template<typename AInt, typename BInt>
auto operator-(const vec3<AInt> &a, const vec3<BInt> &b)-> vec3<decltype(AInt() - BInt())> {
    return {a.x - b.x, a.y - b.y, a.z - b.z};
}

template<typename AInt, typename BInt>
bool operator==(const vec3<AInt> &a, const vec3<BInt> &b) {
    return a.x == b.x && a.y == b.y && a.z == b.z;
}

template<typename AInt, typename BInt>
bool operator<=(const vec3<AInt> &a, const vec3<BInt> &b) {
    return a.x <= b.x && a.y <= b.y && a.z <= b.z;
}

template<typename Int>
std::ostream &operator<<(std::ostream &os, const vec3<Int> &v) {
    os << v.x << "," << v.y << "," << v.z;
    return os;
}

/* A cuboid covering [l, r]. */
template<typename Int>
struct cuboid {
    vec3<Int> l, r;

    cuboid(vec3<Int> l, vec3<Int> r) : l(l), r(r) { }
    cuboid(range<Int> xr, range<Int> yr, range<Int> zr) : cuboid({xr.l, yr.l, zr.l}, {xr.r, yr.r, zr.r}) { }
    cuboid(const cuboid&) = default;

    operator bool() const { return xrange() && yrange() && zrange(); }

    range<Int> xrange() const { return {l.x, r.x}; }
    range<Int> yrange() const { return {l.y, r.y}; }
    range<Int> zrange() const { return {l.z, r.z}; }

    bool contains(vec3<Int> point) const {
        return xrange().contains(point.x) &&
               yrange().contains(point.y) &&
               zrange().contains(point.z);
    }

    template<Int>
    friend std::ostream &operator<<(std::ostream&, const cuboid<Int>&);
};


template<typename Int>
cuboid<Int> operator&(const cuboid<Int> &a, const cuboid<Int> &b) {
    return {
        a.xrange() & b.xrange(),
        a.yrange() & b.yrange(),
        a.zrange() & b.zrange()
    };
}

template<typename AInt, typename BInt>
bool operator==(const cuboid<AInt> &a, const cuboid<BInt> &b) {
    return a.l == b.l && a.r == b.r;
}

template<typename Int>
std::ostream &operator<<(std::ostream &os, const cuboid<Int> &c) {
    os << "x=" << c.l.x << ".." << c.r.x << ","
       << "y=" << c.l.y << ".." << c.r.y << ","
       << "z=" << c.l.z << ".." << c.r.z;
    return os;
}

struct entry {
    size_t id;
    long coord;
    bool start;

    entry(size_t id = 0, long coord = 0, bool start = false)
        : id(id), coord(coord), start(start) { }
};

bool operator<(const entry &l, const entry &r) {
    return l.coord < r.coord ||
        (l.coord == r.coord && l.id < r.id) ||
        (l.coord == r.coord && l.id == r.id && l.start && !r.start);
}

size_t force(const std::vector<std::pair<bool, cuboid<long>>> &inputs) {
    std::vector<entry> xbounds;
    std::vector<entry> ybounds;
    std::vector<entry> zbounds;

    xbounds.reserve(inputs.size() * 2);
    ybounds.reserve(inputs.size() * 2);
    zbounds.reserve(inputs.size() * 2);

    for (size_t i = 0; i < inputs.size(); i++) {
        const auto &input = inputs[i].second;
        xbounds.emplace_back(i, input.l.x, true);
        ybounds.emplace_back(i, input.l.y, true);
        zbounds.emplace_back(i, input.l.z, true);
        xbounds.emplace_back(i, input.r.x, false);
        ybounds.emplace_back(i, input.r.y, false);
        zbounds.emplace_back(i, input.r.z, false);
    }

    std::sort(xbounds.begin(), xbounds.end());
    std::sort(ybounds.begin(), ybounds.end());
    std::sort(zbounds.begin(), zbounds.end());

    std::vector<size_t> results(xbounds.size(), 0);
    std::vector<std::thread> threads(xbounds.size() - 1);

    for (size_t x = 0; x < xbounds.size() - 1; x++) {
        auto concurrent = [xbounds, ybounds, zbounds, inputs, x, &results] () {
            size_t total = 0;
            std::set<size_t> all;
            std::vector<int> xin(inputs.size(), false);
            std::vector<int> yin(inputs.size(), false);
            std::vector<int> zin(inputs.size(), false);

            for (size_t i = 0; i <= x; i++) {
                const auto &xbound = xbounds[i];
                xin[xbound.id] = xbound.start;
            }

            for (size_t y = 0; y < ybounds.size(); y++) {
                const auto &xbound = xbounds[x];
                const auto &ybound = ybounds[y];

                if (ybound.start) {
                    yin[ybound.id] = true;
                    if (xin[ybound.id] && zin[ybound.id])
                        all.insert(ybound.id);
                } else {
                    yin[ybound.id] = false;
                    all.erase(ybound.id);
                }

                for (size_t z = 0; z < zbounds.size(); z++) {
                    const auto &zbound = zbounds[z];

                    if (zbound.start) {
                        zin[zbound.id] = true;
                        if (xin[zbound.id] && yin[zbound.id])
                            all.insert(zbound.id);
                    } else {
                        zin[zbound.id] = false;
                        all.erase(zbound.id);
                    }

                    if (!all.empty() && inputs[*all.rbegin()].first && y + 1 < ybounds.size() && z + 1 < zbounds.size()) {
                        total +=
                            (xbounds[x + 1].coord - xbound.coord) *
                            (ybounds[y + 1].coord - ybound.coord) *
                            (zbounds[z + 1].coord - zbound.coord);
                    }
                }
            }

            results[x] = total;
        };

        std::thread thread(concurrent);
        threads[x] = std::move(thread);
    }

    for (auto &thread : threads)
        thread.join();

    size_t total = 0;
    for (size_t result : results)
        total += result;
    return total;
}

template<typename F>
auto time(const std::string &name, const F &f) {
    auto start = std::chrono::high_resolution_clock::now();
    auto result = f();
    auto time = std::chrono::high_resolution_clock::now() - start;
    std::cout << std::right << std::setw(8) << name << " | "
              << std::left << std::setw(32) << result << " | "
              << std::fixed << std::setprecision(2) << std::right << std::setw(10)
              << std::chrono::duration_cast<std::chrono::microseconds>(time).count() / 1.0e3 << " ms" << std::endl;
    return result;
}

int main() {
    std::vector<std::pair<bool, cuboid<long>>> inputs;

    char action[4];
    long xl, xr, yl, yr, zl, zr;
    while (fscanf(stdin, "%3s x=%ld..%ld,y=%ld..%ld,z=%ld..%ld\n", action, &xl, &xr, &yl, &yr, &zl, &zr) > 0)
        inputs.emplace_back(action[1] == 'n', cuboid<long>({xl, xr + 1}, {yl, yr + 1}, {zl, zr + 1}));

    time("Part 1", [inputs] () {
        cuboid<long> range = {{-50, -50, -50}, {51, 51, 51}};
        std::vector<std::pair<bool, cuboid<long>>> filtered;
        for (const auto &input : inputs)
            if (input.second & range)
                filtered.emplace_back(input.first, input.second & range);
        return force(filtered);
    });

    time("Part 2", [inputs] () {
        return force(inputs);
    });

    return 0;
}
