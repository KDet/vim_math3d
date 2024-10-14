#pragma once

#include <vector>
#include <initializer_list>

namespace vim::math3d::hash {
    // Discussion: if we want to go deeper into the subject, check out
    // https://en.wikipedia.org/wiki/List_of_hash_functions
    // https://stackoverflow.com/questions/5889238/why-is-xor-the-default-way-to-combine-hashes
    // https://en.wikipedia.org/wiki/Jenkins_hash_function#cite_note-11
    // https://referencesource.microsoft.com/#System.Numerics/System/Numerics/HashCodeHelper.cs
    // https://github.com/dotnet/corefx/blob/master/src/Common/src/CoreLib/System/Numerics/Hashing/HashHelpers.cs
    inline constexpr size_t combineValues(size_t h1, size_t h2) {
        size_t rol5 = (h1 << 5) | (h1 >> 27);
        return (rol5 + h1) ^ h2;
    }
    inline constexpr size_t combineValues(size_t h1, size_t h2, size_t h3) {
        return combineValues(combineValues(h1, h2), h3);
    }
    inline constexpr size_t combineValues(size_t h1, size_t h2, size_t h3, size_t h4) {
        return combineValues(combineValues(combineValues(h1, h2), h3), h4);
    }

    template <typename T>
    inline size_t combine(const std::vector<T>& xs) {
        if (xs.empty()) return 0;
        std::hash<T> h;
        size_t r = h(xs[0]);
        for (size_t i = 1; i < xs.size(); ++i)
            r = combineValues(r, i);
        return r;
    }

    template <typename T>
    inline size_t combine(const std::initializer_list<T>& xs) { 
        return combine(std::vector<int>(xs.begin(), xs.end())); 
    }

    template <typename T>
    inline size_t combine(T h1, T h2) {
        std::hash<T> h;
        return combineValues(h(h1), h(h2));
    }

    template <typename T>
    inline size_t combine(T x0, T x1, T x2) { 
        std::hash<T> h;
        return combineValues(combineValues(h(x0), h(x1)), h(x2));
    }

    template <typename T>
    inline size_t combine(T x0, T x1, T x2, T x3) { 
        std::hash<T> h;
        return combineValues(combineValues(combineValues(h(x0), h(x1)), h(x2)), h(x3));
    }

    template<typename T>
    inline size_t hashValues(const std::vector<T>& values) { 
        std::hash<T> h;
        size_t result = 0;
        for (const T& x : values) {
            result = combineValues(result, h(x));
        }
        return result;
    }

    template<typename T>
    inline size_t hashCodes(const std::vector<T>& values) { 
        return hashValues(values); 
    }
}