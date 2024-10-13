#include "test_vim_math3d.h"

int main()
{
#pragma region PlaneTests
    std::cout << "FPlane Tests" << std::endl;

    test("PlaneEqualsTest1", []()
    {
        auto a = FPlane(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FPlane(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        Assert::AreEqual(true, a == b);
        //Assert::AreEqual(true, ((object)a).Equals(b));

        // case 2: compare between different values
        auto c = FPlane(FVector3(10.0f, b.Normal.Y, b.Normal.Z), b.D);
        Assert::AreEqual(false, b == c);
        Assert::AreEqual(false, b == c);
    });

    // A test for operator == (FPlane, FPlane)
        
    test("PlaneEqualityTest", []()
    {
        auto a = FPlane(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FPlane(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = FPlane(FVector3(10.0f, b.Normal.Y, b.Normal.Z), b.D);
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for FPlane (float, float, float, float)
        
    test("PlaneConstructorTest1", []()
    {
        float a = 1.0f, b = 2.0f, c = 3.0f, d = 4.0f;
        auto target = FPlane(a, b, c, d);

        Assert::True(
            target.Normal.X == a && target.Normal.Y == b && target.Normal.Z == c && target.D == d,
            "FPlane.cstor did not return the expected value.");
    });

    // A test for FPlane.CreateFromVertices
        
    test("PlaneCreateFromVerticesTest", []()
    {
        auto point1 = FVector3(0.0f, 1.0f, 1.0f);
        auto point2 = FVector3(0.0f, 0.0f, 1.0f);
        auto point3 = FVector3(1.0f, 0.0f, 1.0f);

        auto target = FPlane(point1, point2, point3);
        auto expected = FPlane(FVector3(0, 0, 1), -1.0f);
        Assert::True(target == expected, "FPlane operator== did not return the expected value.");
    });

        // A test for FPlane.CreateFromVertices
        
        test("PlaneCreateFromVerticesTest2", []()
        {
            auto point1 = FVector3(0.0f, 0.0f, 1.0f);
            auto point2 = FVector3(1.0f, 0.0f, 0.0f);
            auto point3 = FVector3(1.0f, 1.0f, 0.0f);

            auto target = FPlane(point1, point2, point3);
            auto invRoot2 = (float)(1 / std::sqrt(2));

            auto expected = FPlane(FVector3(invRoot2, 0, invRoot2), -invRoot2);
            Assert::True(MathHelper::Equal(target, expected), "FPlane.cstor did not return the expected value.");
        });

        // A test for FPlane (Vector3f, float)
        
        test("PlaneConstructorTest3", []()
        {
            auto normal = FVector3(1, 2, 3);
            float d = 4;

            auto target = FPlane(normal, d);
            Assert::True(
                target.Normal == normal && target.D == d,
                "FPlane.cstor did not return the expected value.");
        });

        // A test for FPlane (Vector4f)
        
        test("PlaneConstructorTest", []()
        {
            auto value = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
            auto target = FPlane(value);

            Assert::True(
                target.Normal.X == value.X && target.Normal.Y == value.Y && target.Normal.Z == value.Z && target.D == value.W,
                "FPlane.cstor did not return the expected value.");
        });

        
        test("PlaneDotTest", []()
        {
            auto target = FPlane(2, 3, 4, 5);
            auto value = FVector4(5, 4, 3, 2);

            float expected = 10 + 12 + 12 + 10;
            auto actual = FPlane::dot(target, value);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.Dot returns unexpected value.");
        });

        
        test("PlaneDotCoordinateTest", []()
        {
            auto target = FPlane(2, 3, 4, 5);
            auto value = FVector3(5, 4, 3);

            float expected = 10 + 12 + 12 + 5;
            auto actual = FPlane::dotCoordinate(target, value);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.DotCoordinate returns unexpected value.");
        });

        
        test("PlaneDotNormalTest", []()
        {
            auto target = FPlane(2, 3, 4, 5);
            auto value = FVector3(5, 4, 3);

            float expected = 10 + 12 + 12;
            auto actual = FPlane::dotNormal(target, value);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.DotCoordinate returns unexpected value.");
        });

        
        test("PlaneNormalizeTest", []()
        {
            auto target = FPlane(1, 2, 3, 4);

            auto f = target.Normal.lengthSquared();
            auto invF = 1.0f / mathOps::sqrt(f); // f.Sqrt();
            auto expected = FPlane(target.Normal * invF, target.D * invF);

            auto actual = FPlane::normalize(target);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.Normalize returns unexpected value.");

            // normalize, normalized normal.
            actual = FPlane::normalize(actual);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.Normalize returns unexpected value.");
        });

        
        // Transform by matrix
        test("PlaneTransformTest1", []()
        {
            auto target = FPlane(1, 2, 3, 4);
            target = FPlane::normalize(target);

            auto m =
                FMatrix4x4::rotationX(MathHelper::ToRadians(30.0)) *
                FMatrix4x4::rotationY(MathHelper::ToRadians(30.0)) *
                FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0));
            m.M41 = 10.0f;
            m.M42 = 20.0f;
            m.M43 = 30.0f;

            FMatrix4x4 inv = m.invert().value_or(FMatrix4x4(std::numeric_limits<float>::quiet_NaN()));
            auto itm = FMatrix4x4::transpose(inv);
            float x = target.Normal.X, y = target.Normal.Y, z = target.Normal.Z, w = target.D;
            auto Normal = FVector3(
                x * itm.M11 + y * itm.M21 + z * itm.M31 + w * itm.M41,
                x * itm.M12 + y * itm.M22 + z * itm.M32 + w * itm.M42,
                x * itm.M13 + y * itm.M23 + z * itm.M33 + w * itm.M43);
            auto D = x * itm.M14 + y * itm.M24 + z * itm.M34 + w * itm.M44;
            auto expected = FPlane(Normal, D);
            auto actual = FMatrix4x4::transform(target, m);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.Transform did not return the expected value.");
        });

        
        // Transform by quaternion
        test("PlaneTransformTest2", []()
        {
            auto target = FPlane(1, 2, 3, 4);
            target = FPlane::normalize(target);

            auto m =
                FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
                FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
                FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
            auto q = m.quaternion();

            float x = target.Normal.X, y = target.Normal.Y, z = target.Normal.Z, w = target.D;
            auto Normal = FVector3(
                x * m.M11 + y * m.M21 + z * m.M31 + w * m.M41,
                x * m.M12 + y * m.M22 + z * m.M32 + w * m.M42,
                x * m.M13 + y * m.M23 + z * m.M33 + w * m.M43);
            auto D = x * m.M14 + y * m.M24 + z * m.M34 + w * m.M44;
            auto expected = FPlane(Normal, D);
            auto actual = FMatrix4x4::transform(target, m);
            Assert::True(MathHelper::Equal(expected, actual), "FPlane.Transform did not return the expected value.");
        });

        // A test for FPlane comparison involving NaN values
        
        test("PlaneEqualsNanTest", []() {
            const auto nan = std::numeric_limits<float>::quiet_NaN();
            auto a = FPlane(nan, 0, 0, 0);
            auto b = FPlane(0, nan, 0, 0);
            auto c = FPlane(0, 0, nan, 0);
            auto d = FPlane(0, 0, 0, nan);

            Assert::False(a == FPlane(0, 0, 0, 0));
            Assert::False(b == FPlane(0, 0, 0, 0));
            Assert::False(c == FPlane(0, 0, 0, 0));
            Assert::False(d == FPlane(0, 0, 0, 0));

            Assert::True(a != FPlane(0, 0, 0, 0));
            Assert::True(b != FPlane(0, 0, 0, 0));
            Assert::True(c != FPlane(0, 0, 0, 0));
            Assert::True(d != FPlane(0, 0, 0, 0));

            Assert::False(a == (FPlane(0, 0, 0, 0)));
            Assert::False(b == (FPlane(0, 0, 0, 0)));
            Assert::False(c == (FPlane(0, 0, 0, 0)));
            Assert::False(d == (FPlane(0, 0, 0, 0)));

            // Counterintuitive result - IEEE rules for NaN comparison are weird!
            Assert::False(a == a);
            Assert::False(b == b);
            Assert::False(c == c);
            Assert::False(d == d);
        });


#pragma endregion

    return 0;
}