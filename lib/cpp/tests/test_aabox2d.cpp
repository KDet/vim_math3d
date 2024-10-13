#include "test_vim_math3d.h"

int main()
{
#pragma region AABox2DTests
    std::cout << "AABox2D Tests" << std::endl;

    test("TestNoIntersection1", []() {
        auto box1 = AABox2D(FVector2(0, 0), FVector2(3, 3));
        auto box2 = AABox2D(FVector2(4, 4), FVector2(5, 5));
        Assert::IsFalse(box1.intersects(box2));
        Assert::IsFalse(box2.intersects(box1));
     });

    test("TestNoIntersection2", []() {
        auto box1 = AABox2D(FVector2(0, 0), FVector2(3, 3));
        auto box2 = AABox2D(FVector2(-5, -5), FVector2(-4, -4));
        Assert::IsFalse(box1.intersects(box2));
        Assert::IsFalse(box2.intersects(box1));
    });

    // Intersection tests

    test("TestIntersects1", []() {
        auto box1 = AABox2D(FVector2(0, 0), FVector2(5, 5));
        auto box2 = AABox2D(FVector2(1, 1), FVector2(2, 2));
        Assert::IsTrue(box1.intersects(box2));
        Assert::IsTrue(box2.intersects(box1));
    });

    test("TestIntersects2", []() {
        auto box1 = AABox2D(FVector2(0, 0), FVector2(3, 3));
        auto box2 = AABox2D(FVector2(1, -1), FVector2(2, 7));
        Assert::IsTrue(box1.intersects(box2));
        Assert::IsTrue(box2.intersects(box1));
    });

    test("TestIntersects3", []() {
        auto  box1 = AABox2D(FVector2(0, 0), FVector2(3, 3));
        auto  box2 = AABox2D(FVector2(1, -1), FVector2(2, 2));
        Assert::IsTrue(box1.intersects(box2));
        Assert::IsTrue(box2.intersects(box1));
    });

    test("TestIntersects4", []() {
        auto box1 = AABox2D(FVector2(0, 0), FVector2(3, 3));
        auto box2 = AABox2D(FVector2(3, 3), FVector2(5, 5));
        Assert::IsTrue(box1.intersects(box2));
        Assert::IsTrue(box2.intersects(box1));
    });

#pragma endregion
 
    return 0;
}