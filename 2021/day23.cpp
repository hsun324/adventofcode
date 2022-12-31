#include <array>
#include <chrono>
#include <cstdio>
#include <iomanip>
#include <iostream>
#include <optional>
#include <queue>
#include <unordered_map>
#include <vector>

enum kind {
    KIND_A = 0,
    KIND_B = 1,
    KIND_C = 2,
    KIND_D = 3,

    NUM_KINDS = 4,

    EMPTY = -1
};

constexpr size_t hallway_size = 11;
constexpr std::array<size_t, NUM_KINDS> exits = {{2, 4, 6, 8}};
constexpr std::array<size_t, NUM_KINDS> movement_cost = {{1, 10, 100, 1000}};
constexpr std::array<bool, hallway_size> invalid = {{false, false, true, false, true, false, true, false, true, false, false}};

template<size_t RoomSize>
struct state {
    std::array<kind, hallway_size> hallway;
    std::array<std::array<kind, RoomSize>, NUM_KINDS> rooms;

    state(const std::array<std::array<kind, RoomSize>, NUM_KINDS> &rooms = {}) : rooms(rooms) {
        hallway.fill(EMPTY);
    }
    state(const state&) = default;
    state(state&&) = default;

    state &operator=(const state&) = default;
    state &operator=(state&&) = default;

    bool operator==(const state&) const = default;

    template<typename T>
    void step(T push) const {
        for (size_t i = 0; i < hallway.size(); i++) {
            if (hallway[i] != EMPTY) {
                kind k = hallway[i];
                size_t dest = exits[k];
                if (!blocked(i, dest) && stable(k)) {
                    size_t n = next(k);
                    size_t cost = (dist(i, dest) + RoomSize - n) * movement_cost[k];
                    state next = *this;
                    std::swap(next.rooms[k][n], next.hallway[i]);
                    push(std::move(next), cost);
                }
            }
        }

        for (size_t room = 0; room < NUM_KINDS; room++) {
            if (!stable(room)) {
                size_t t = top(room);
                kind k = rooms[room][t];
                for (size_t i = 0; i < hallway_size; i++) {
                    if (!invalid[i] && !blocked(exits[room], i)) {
                        size_t cost = (RoomSize - t + dist(exits[room], i)) * movement_cost[k];
                        state next = *this;
                        std::swap(next.hallway[i], next.rooms[room][t]);
                        push(std::move(next), cost);
                    }
                }
            }
        }
    }

    inline size_t dist(size_t start, size_t end) const {
        return std::max(start, end) - std::min(start, end);
    }

    bool stable(size_t room) const {
        for (size_t i = 0; i < RoomSize; i++) {
            if (rooms[room][i] == EMPTY)
                return true;
            if (rooms[room][i] != room)
                return false;
        }
        return true;
    }
    bool filled(size_t room) const {
        return rooms[room][RoomSize - 1] != EMPTY;
    }

    size_t top(size_t room) const {
        for (size_t i = RoomSize - 1; i < RoomSize; i--)
            if (rooms[room][i] != EMPTY)
                return i;
        abort();
    }
    size_t next(size_t room) const {
        for (size_t i = 0; i < RoomSize; i++)
            if (rooms[room][i] == EMPTY)
                return i;
        abort();
    }

    bool blocked(size_t start, size_t end) const {
        for (size_t i = std::min(start, end); i <= std::max(start, end); i++)
            if (i != start && hallway[i] != EMPTY)
                return true;
        return false;
    }

    bool finished() const {
        for (size_t room = 0; room < NUM_KINDS; room++)
            if (!stable(room) || !filled(room))
                return false;
        return true;
    }

    template<size_t S>
    friend std::ostream &operator<<(std::ostream&, const state<S>&);
};

template<size_t RoomSize>
std::ostream &operator<<(std::ostream &os, const state<RoomSize> &s) {
    for (auto space : s.hallway)
        std::cout << (char) (space == EMPTY ? '.' : (space + 'A'));
    for (const auto &room : s.rooms) {
        std::cout << " | ";
        for (auto space : room)
            std::cout << (char) (space == EMPTY ? '.' : (space + 'A'));
    }
    return os;
}

template<size_t RoomSize>
struct entry {
    state<RoomSize> current;
    state<RoomSize> prev;
    int cost;
    int penalty;

    entry(const state<RoomSize> &current, const state<RoomSize> &prev = {}, int cost = 0)
        : current(current), prev(prev), cost(cost) { normalize(); }
    entry(state<RoomSize> &&current, const state<RoomSize> &prev = {}, int cost = 0)
        : current(current), prev(prev), cost(cost) { normalize(); }

    entry(const entry&) = default;
    entry(entry&&) = default;

    entry &operator=(const entry&) = default;
    entry &operator=(entry&&) = default;

    template<typename Queue>
    void step(Queue &queue) const {
        current.step([&] (state<RoomSize> &&next, size_t price) {
            queue.emplace(next, current, cost + price);
        });
    }

    void normalize() {
        penalty = 0;
        for (size_t i = 0; i < current.hallway.size(); i++)
            if (current.hallway[i] != EMPTY)
                penalty += (std::max(i, exits[i]) - std::min(i, exits[i]) + 1) * movement_cost[current.hallway[i]];
        for (size_t i = 0; i < NUM_KINDS; i++)
            for (size_t j = 0; j < RoomSize; j++)
                if (current.rooms[i][j] != i && current.rooms[i][j] != EMPTY)
                    penalty += (RoomSize - j + 3) * movement_cost[current.rooms[i][j]];
    }

    template<size_t S>
    friend std::ostream &operator<<(std::ostream&, const entry<S>&);
};

namespace std {
    template<size_t RowSize>
    struct hash<state<RowSize>> {
        size_t operator()(const state<RowSize> &s) const {
            size_t seed = RowSize;
            for (auto space : s.hallway)
                seed ^= space + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            for (const auto &room : s.rooms)
                for (auto space : room)
                    seed ^= space + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            return seed;
        }
    };

    template<size_t RowSize>
    struct hash<entry<RowSize>> {
        size_t operator()(const entry<RowSize> &s) const {
            size_t seed = 2;
            size ^= std::hash<state<RowSize>>{}(s.current) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            size ^= std::hash<size_t>{}(s.cost) + 0x9e3779b9 + (seed << 6) + (seed >> 2);
            return seed;
        }
    };
};

template<size_t RoomSize>
std::ostream &operator<<(std::ostream &os, const entry<RoomSize> &s) {
    std::cout << s.current << " | " << std::setw(6) << s.cost << " | " << std::setw(6) << s.penalty;
    return os;
}

template<size_t RoomSize>
bool operator==(const entry<RoomSize> &l, const entry<RoomSize> &r) {
    return l.current == r.current && l.cost == r.cost;
}

template<size_t RoomSize>
bool operator<(const entry<RoomSize> &r, const entry<RoomSize> &l) {
    return (r.cost + r.penalty) > (l.cost + l.penalty);
}

template<size_t RoomSize>
std::optional<std::pair<size_t, std::vector<state<RoomSize>>>> solve(const state<RoomSize> &initial) {
    std::unordered_map<state<RoomSize>, state<RoomSize>> visited;
    std::priority_queue<entry<RoomSize>> queue;
    queue.push(initial);

    while (!queue.empty()) {
        entry top = queue.top();
        queue.pop();

        // std::cout << "VISITING | " << current << std::endl;

        if (visited.contains(top.current))
            continue;
        visited.emplace(top.current, top.prev);

        if (top.current.finished()) {
            std::vector<state<RoomSize>> path;
            for (state<RoomSize> &node = top.current; visited.contains(node); node = visited[node])
                path.push_back(node);
            return std::make_pair(top.cost, path);
        }

        top.step(queue);
    }
    return std::nullopt;
}

template<typename F>
auto time(const std::string &name, const F &f) {
    auto start = std::chrono::high_resolution_clock::now();
    auto result = f();
    auto time = std::chrono::high_resolution_clock::now() - start;
    std::cout << name << ": " << result << " - "
              << (float) std::chrono::duration_cast<std::chrono::microseconds>(time).count() / 1e3 << "ms" << std::endl;
    return result;
}

int main() {
    time("Part 1", [] () {
        std::array<kind, 2> room0 = {{KIND_D, KIND_D}};
        std::array<kind, 2> room1 = {{KIND_A, KIND_A}};
        std::array<kind, 2> room2 = {{KIND_B, KIND_C}};
        std::array<kind, 2> room3 = {{KIND_B, KIND_C}};
        state<2> initial({room0, room1, room2, room3});
        std::pair<size_t, std::vector<state<2>>> solution = *solve(initial);

        for (auto iter = solution.second.rbegin(), end = solution.second.rend(); iter != end; ++iter)
            std::cout << *iter << std::endl;
        return solution.first;
    });

    time("Part 2", [] () {
        std::array<kind, 4> room0 = {{KIND_D, KIND_D, KIND_D, KIND_D}};
        std::array<kind, 4> room1 = {{KIND_A, KIND_B, KIND_C, KIND_A}};
        std::array<kind, 4> room2 = {{KIND_B, KIND_A, KIND_B, KIND_C}};
        std::array<kind, 4> room3 = {{KIND_B, KIND_C, KIND_A, KIND_C}};
        state<4> initial({room0, room1, room2, room3});
        std::pair<size_t, std::vector<state<4>>> solution = *solve(initial);

        for (auto iter = solution.second.rbegin(), end = solution.second.rend(); iter != end; ++iter)
            std::cout << *iter << std::endl;
        return solution.first;
    });

    return 0;
}
