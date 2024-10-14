#include "test_vim_math3d.h"

int main()
{ 
#pragma region RayTests
    std::cout << "Ray Tests" << std::endl;

    test("Ray_IntersectBox_IsFalse_OutsideBox", []()
    {
        auto ray = Ray(FVector3(-2, 0, -2), FVector3(0, 0, 1));
        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));

        Assert::True(ray.intersects(box) == std::nullopt);
    });
    test("Ray_IntersectBox_IsTrue_Through", []()
    {
        auto front = Ray(FVector3(0, 0, -2), FVector3(0, 0, 1));
        auto back = Ray(FVector3(0, 0, 2), FVector3(0, 0, -1));
        auto left = Ray(FVector3(-2, 0, 0), FVector3(1, 0, 0));
        auto right = Ray(FVector3(2, 0, 0), FVector3(-1, 0, 0));
        auto top = Ray(FVector3(0, 2, 0), FVector3(0, -1, 0));
        auto under = Ray(FVector3(0, -2, 0), FVector3(0, 1, 0));

        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));

        Assert::True(front.intersects(box) != std::nullopt, "front is null");
        Assert::True(back.intersects(box) != std::nullopt, "back is null");
        Assert::True(left.intersects(box) != std::nullopt, "left is null");
        Assert::True(right.intersects(box) != std::nullopt, "right is null");
        Assert::True(top.intersects(box) != std::nullopt, "top is null");
        Assert::True(under.intersects(box) != std::nullopt, "under is null");
        
    });
    test("Ray_IntersectBox_IsTrue_ThroughDiagonals", []()
    {
        auto XYnZ = Ray(FVector3(2, 2, -2), FVector3(-1, -1, 1));
        auto nXYnZ = Ray(FVector3(-2, 2, -2), FVector3(1, -1, 1));
        auto nXnYnZ = Ray(FVector3(-2, -2, -2), FVector3(1, 1, 1));
        auto XnYnZ = Ray(FVector3(2, -2, -2), FVector3(-1, 1, 1));

        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));

        Assert::True(XYnZ.intersects(box) != std::nullopt, "XYnZ is null");
        Assert::True(nXYnZ.intersects(box) != std::nullopt, "nXYnZ is null");
        Assert::True(nXnYnZ.intersects(box) != std::nullopt, "nXnYnZ is null");
        Assert::True(XnYnZ.intersects(box) != std::nullopt, "XnYnZ is null");

    });
    test("Ray_IntersectBox_IsFalse_AwayFromBox", []()
    {
        auto front = Ray(FVector3(0, 0, -2), FVector3(0, 0, -1));
        auto back = Ray(FVector3(0, 0, 2), FVector3(0, 0, 1));
        auto left = Ray(FVector3(-2, 0, 0), FVector3(-1, 0, 0));
        auto right = Ray(FVector3(2, 0, 0), FVector3(1, 0, 0));
        auto top = Ray(FVector3(0, 2, 0), FVector3(0, 1, 0));
        auto under = Ray(FVector3(0, -2, 0), FVector3(0, -1, 0));

        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));
        
        Assert::True(front.intersects(box) == std::nullopt, "front is not null");
        Assert::True(back.intersects(box) == std::nullopt, "back is not null");
        Assert::True(left.intersects(box) == std::nullopt, "left is not null");
        Assert::True(right.intersects(box) == std::nullopt, "right is not null");
        Assert::True(top.intersects(box) == std::nullopt, "top is not null");
        Assert::True(under.intersects(box) == std::nullopt, "under is not null");
    });
    test("Ray_IntersectBox_IsTrue_OnEdge", []()
    {
        auto front = Ray(FVector3(0, 2, -1), FVector3(0, -1, 0));
        auto back = Ray(FVector3(0, 2, 1), FVector3(0, -1, 0));
        auto left = Ray(FVector3(-1, 0, -2), FVector3(0, 0, 1));
        auto right = Ray(FVector3(1, 0, -2), FVector3(0, 0, 1));
        auto top = Ray(FVector3(0, 1, -2), FVector3(0, 0, 1));
        auto under = Ray(FVector3(0, -1, -2), FVector3(0, 0, 1));

        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));

        Assert::True(front.intersects(box) != std::nullopt, "front is null");
        Assert::True(back.intersects(box) != std::nullopt, "back is null");
        Assert::True(left.intersects(box) != std::nullopt, "left is null");
        Assert::True(right.intersects(box) != std::nullopt, "right is null");
        Assert::True(top.intersects(box) != std::nullopt, "top is null");
        Assert::True(under.intersects(box) != std::nullopt, "under is null");
    });
    test("Ray_IntersectBox_IsFalse_NearEdge", []()
    {
        auto ray = Ray(FVector3(0, 0, -2), FVector3(0, 1.1f, 1));
        auto box = AABox(FVector3(-1, -1, -1), FVector3(1, 1, 1));

        Assert::True(ray.intersects(box) == std::nullopt, "ray is not null");
    });
    test("Ray_IntersectBox_IsTrue_FlatBox", []()
    {
        auto box = AABox(FVector3(-1, -1, 0), FVector3(1, 1, 0));
        auto ray = Ray(FVector3(0, 0, -1), FVector3(0, 0, 1));

        Assert::True(ray.intersects(box) != std::nullopt, "ray is null");
    });
    test("Ray_IntersectTriangle_IsTrue_Inside", []()
    {
        auto ray = Ray(FVector3(0, 0, -1), FVector3(0, 0, 1));

        auto triangle = Triangle(
            FVector3(0, 1, 0),
            FVector3(1, -1, 0),
            FVector3(-1, -1, 0)
        );

        Assert::True(ray.intersects(triangle) != std::nullopt, "ray is null");
    });
    test("Ray_IntersectTriangle_IsFalse_Parralel", []()
    {
        auto ray = Ray(FVector3(0, 0, -1), FVector3(0, 0, 1));

        auto triangle = Triangle(
            FVector3(1, 0, 0),
            FVector3(-1, 0, 0),
            FVector3(0, 0, 1)
        );

        Assert::True(ray.intersects(triangle) == std::nullopt, "ray is not null");
    });
    test("Ray_IntersectTriangle_IsTrue_OnCorner", []()
    {
        auto ray = Ray(FVector3(0, 1, -1), FVector3(0, 0, 1));

        auto triangle = Triangle(
            FVector3(0, 1, 0),
            FVector3(1, -1, 0),
            FVector3(-1, -1, 0)
        );

        Assert::True(ray.intersects(triangle) != std::nullopt, "ray is null");
    });
    test("Ray_IntersectTriangle_IsFalse_InTrickyCorner", []()
    {
        const auto ray = Ray(FVector3(-0.1f, 0, -1), FVector3(0, 0, 1));
        const auto triangle = Triangle(FVector3(0, 0, 0), FVector3(-1, 1, 0), FVector3(1, 0, 0));

        Assert::True(ray.intersects(triangle) == std::nullopt, "ray is not null");
    });
    test("Ray_IntersectTriangle_PerfTest", []()
    {
        //IsFalse_InTrickyCorner
        const auto ray1 = Ray(FVector3(-0.1f, 0, -1), FVector3(0, 0, 1));
        const auto triangle1 = Triangle(FVector3(0, 0, 0), FVector3(-1, 1, 0), FVector3(1, 0, 0));

        //IsTrue_OnCorner
        const auto ray2 = Ray(FVector3(0, 1, -1), FVector3(0, 0, 1));
        const auto triangle2 = Triangle(FVector3(0, 1, 0), FVector3(1, -1, 0), FVector3(-1, -1, 0));

        //IsTrue_OnCorner
        const auto ray3 = Ray(FVector3(0, 0, -1), FVector3(0, 0, 1));
        const auto triangle3 = Triangle(FVector3(1, 0, 0), FVector3(-1, 0, 0), FVector3(0, 0, 1));

        // IsFalse_Parralel
        const auto ray4 = Ray(FVector3(0, 0, -1), FVector3(0, 0, 1));
        const auto triangle4 = Triangle(FVector3(1, 0, 0), FVector3(-1, 0, 0), FVector3(0, 0, 1));

        std::vector<std::optional<float>> values;
        for (auto j = 0; j < 10; j++)
        {
            auto watch = std::chrono::high_resolution_clock::now();
            for (auto i = 0; i < 1000000; i++)
            {
                values.push_back(intersects(ray1, triangle1));
                values.push_back(intersects(ray2, triangle2));
                values.push_back(intersects(ray3, triangle3));
                values.push_back(intersects(ray4, triangle4));
            }

            auto thombore = std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::high_resolution_clock::now() - watch).count();
            std::cout << "TomboreMoller " << thombore << " ms. " << values.size() << " intersects." << std::endl;
        }
    });

#pragma endregion

    return 0;
}