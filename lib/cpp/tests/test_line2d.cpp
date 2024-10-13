#include "test_vim_math3d.h"

int main()
{
#pragma region Line2DTests
    std::cout << "Line2D Tests" << std::endl;

    test("TestNoIntersection1", []()
    {
        auto a =  Line2D( FVector2(0, 0),  FVector2(7, 7));
        auto b =  Line2D( FVector2(3, 4),  FVector2(4, 5));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection2", []()
    {
        auto a =  Line2D( FVector2(-4, 4),  FVector2(-2, 1));
        auto b =  Line2D( FVector2(-2, 3),  FVector2(0, 0));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection3", []()
    {
        auto a =  Line2D( FVector2(0, 0),  FVector2(0, 1));
        auto b =  Line2D( FVector2(2, 2),  FVector2(2, 3));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection4", []()
    {
        auto a =  Line2D( FVector2(0, 0),  FVector2(0, 1));
        auto b =  Line2D( FVector2(2, 2),  FVector2(3, 2));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection5", []()
    {
        auto a =  Line2D( FVector2(-1, -1),  FVector2(2, 2));
        auto b =  Line2D( FVector2(3, 3),  FVector2(5, 5));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection6", []()
    {
        auto a =  Line2D( FVector2(0, 0),  FVector2(1, 1));
        auto b =  Line2D( FVector2(2, 0),  FVector2(0.5f, 2));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection7", []()
    {
        auto a =  Line2D( FVector2(1, 1),  FVector2(4, 1));
        auto b =  Line2D( FVector2(2, 2),  FVector2(3, 2));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });


    test("TestNoIntersection8", []()
    {
        auto a =  Line2D( FVector2(0, 5),  FVector2(6, 0));
        auto b =  Line2D( FVector2(2, 1),  FVector2(2, 2));
        Assert::IsFalse(a.intersects(b));
        Assert::IsFalse(b.intersects(a));
    });

    // Intersection tests


    test("TestIntersects1", []()
    {
        auto a =  Line2D( FVector2(0, -2),  FVector2(0, 2));
        auto b =  Line2D( FVector2(-2, 0),  FVector2(2, 0));
        Assert::IsTrue(a.intersects(b));
        Assert::IsTrue(b.intersects(a));
    });


    test("TestIntersects2", []()
    {
        auto a =  Line2D( FVector2(5, 5),  FVector2(0, 0));
        auto b =  Line2D( FVector2(1, 1),  FVector2(8, 2));
        Assert::IsTrue(a.intersects(b));
        Assert::IsTrue(b.intersects(a));
    });


    test("TestIntersects3", []()
    {
        auto a =  Line2D( FVector2(-1, 0),  FVector2(0, 0));
        auto b =  Line2D( FVector2(-1, -1),  FVector2(-1, 1));
        Assert::IsTrue(a.intersects(b));
        Assert::IsTrue(b.intersects(a));
    });


    test("TestIntersects4", []()
    {
        auto a =  Line2D( FVector2(0, 2),  FVector2(2, 2));
        auto b =  Line2D( FVector2(2, 0),  FVector2(2, 4));
        Assert::IsTrue(a.intersects(b));
        Assert::IsTrue(b.intersects(a));
    });


    test("TestIntersects5", []()
    {
        auto a =  Line2D( FVector2(0, 0),  FVector2(5, 5));
        auto b =  Line2D( FVector2(1, 1),  FVector2(3, 3));
        Assert::IsTrue(a.intersects(b));
        Assert::IsTrue(b.intersects(a));
    });


    test("TestIntersects6", []()
    {
        srand(time(nullptr)); // Seed the random number generator with the current time
    
        for (auto i = 0; i < 50; i++)
        {
            auto ax = static_cast<double>(rand()) / RAND_MAX;
            auto ay = static_cast<double>(rand()) / RAND_MAX;
            auto bx = static_cast<double>(rand()) / RAND_MAX;
            auto by = static_cast<double>(rand()) / RAND_MAX;
            auto a =  Line2D( FVector2(ax, ay),  FVector2(bx, by));
            auto b =  Line2D( FVector2(ax, ay),  FVector2(bx, by));
            Assert::IsTrue(a.intersects(b));
            Assert::IsTrue(b.intersects(a));
        }
    });
#pragma endregion

    return 0;
}