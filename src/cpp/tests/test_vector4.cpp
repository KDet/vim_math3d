#include "test_vim_math3d.h"

int main()
{
 
#pragma region Vector4Tests
    std::cout << "FVector4 Tests" << std::endl;
    
    test("Vector4MarshalSizeTest", []()
    {
        auto v = FVector4();

        Assert::True(16 == sizeof(FVector4));
        Assert::True(16 == sizeof(struct FVector4));
        Assert::True(16 == sizeof(v));
    });

    test("Vector4hashTest", []()
    {
        auto v1 = FVector4(2.5f, 2.0f, 3.0f, 3.3f);
        auto v2 = FVector4(2.5f, 2.0f, 3.0f, 3.3f);
        auto v3 = FVector4(2.5f, 2.0f, 3.0f, 3.3f);
        auto v5 = FVector4(3.3f, 3.0f, 2.0f, 2.5f);
        Assert::AreEqual(v1.hash(), v1.hash());
        Assert::AreEqual(v1.hash(), v2.hash());
        Assert::AreNotEqual(v1.hash(), v5.hash());
        Assert::AreEqual(v1.hash(), v3.hash());
        auto v4 = FVector4(0.0f, 0.0f, 0.0f, 0.0f);
        auto v6 = FVector4(1.0f, 0.0f, 0.0f, 0.0f);
        auto v7 = FVector4(0.0f, 1.0f, 0.0f, 0.0f);
        auto v8 = FVector4(1.0f, 1.0f, 1.0f, 1.0f);
        auto v9 = FVector4(1.0f, 1.0f, 0.0f, 0.0f);
        Assert::AreNotEqual(v4.hash(), v6.hash());
        Assert::AreNotEqual(v4.hash(), v7.hash());
        Assert::AreNotEqual(v4.hash(), v8.hash());
        Assert::AreNotEqual(v7.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v7.hash());
        Assert::AreNotEqual(v9.hash(), v7.hash());
    });

    // A test for DistanceSquared (Vector4f, Vector4f)
        
    test("Vector4DistanceSquaredTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = 64.0f;
        float actual;

        actual = a.distanceSquared(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.DistanceSquared did not return the expected value.");
    });

    // A test for Distance (Vector4f, Vector4f)

    test("Vector4DistanceTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = 8.0f;
        float actual;

        actual = a.distance(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Distance did not return the expected value.");
    });

    // A test for Distance (Vector4f, Vector4f)
    // Distance from the same point

    test("Vector4DistanceTest1", []()
    {
        auto a = FVector4(FVector2(1.051f, 2.05f), 3.478f, 1.0f);
        auto b = FVector4(FVector3(1.051f, 2.05f, 3.478f), 0.0f);
        b = b.setW(1);

        auto actual = a.distance(b);
        Assert::AreEqual(0.0f, actual);
    });

    // A test for Dot (Vector4f, Vector4f)

    test("Vector4DotTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = 70.0f;
        float actual;

        actual = a.dot(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Dot did not return the expected value.");
    });

    // A test for Dot (Vector4f, Vector4f)
    // Dot test for perpendicular vector

    test("Vector4DotTest1", []()
    {
        auto a = FVector3(1.55f, 1.55f, 1);
        auto b = FVector3(2.5f, 3, 1.5f);
        auto c = a.cross(b);

        auto d = FVector4(a, 0);
        auto e = FVector4(c, 0);

        auto actual = d.dot(e);
        Assert::True(MathHelper::Equal(0.0f, actual), "Vector4f.Dot did not return the expected value.");
    });

    // A test for Length ()

    test("Vector4LengthTest", []()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto w = 4.0f;

        auto target = FVector4(a, w);

        auto expected = (float)std::sqrt(30.0f);
        float actual;

        actual = target.length();

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Length did not return the expected value.");
    });

    // A test for Length ()
    // Length test where length is zero

    test("Vector4LengthTest1", []()
    {
        auto target = FVector4();

        auto expected = 0.0f;
        auto actual = target.length();

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Length did not return the expected value.");
    });

    // A test for LengthSquared ()

    test("Vector4LengthSquaredTest", []()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto w = 4.0f;

        auto target = FVector4(a, w);

        float expected = 30;
        float actual;

        actual = target.lengthSquared();

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.LengthSquared did not return the expected value.");
    });

    // A test for Min (Vector4f, Vector4f)

    test("Vector4MinTest", []()
    {
        auto a = FVector4(-1.0f, 4.0f, -3.0f, 1000.0f);
        auto b = FVector4(2.0f, 1.0f, -1.0f, 0.0f);

        auto expected = FVector4(-1.0f, 1.0f, -3.0f, 0.0f);
        FVector4 actual;
        actual = a.minimum(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Min did not return the expected value.");
    });

    // A test for Max (Vector4f, Vector4f)

    test("Vector4MaxTest", []()
    {
        auto a = FVector4(-1.0f, 4.0f, -3.0f, 1000.0f);
        auto b = FVector4(2.0f, 1.0f, -1.0f, 0.0f);

        auto expected = FVector4(2.0f, 4.0f, -1.0f, 1000.0f);
        FVector4 actual;
        actual = a.maximum(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Max did not return the expected value.");
    });


    test("Vector4MinMaxCodeCoverageTest", []()
    {
        auto min = FVector4::zero();
        auto max = FVector4::one();
        FVector4 actual;

        // Min.
        actual = min.minimum(max);
        Assert::AreEqual(actual, min);

        actual = max.minimum(min);
        Assert::AreEqual(actual, min);

        // Max.
        actual = min.maximum(max);
        Assert::AreEqual(actual, max);

        actual = max.maximum(min);
        Assert::AreEqual(actual, max);
    });

    // A test for Clamp (Vector4f, Vector4f, Vector4f)

    test("Vector4ClampTest", []()
    {
        auto a = FVector4(0.5f, 0.3f, 0.33f, 0.44f);
        auto min = FVector4(0.0f, 0.1f, 0.13f, 0.14f);
        auto max = FVector4(1.0f, 1.1f, 1.13f, 1.14f);

        // Normal case.
        // Case N1: specified value is in the range.
        auto expected = FVector4(0.5f, 0.3f, 0.33f, 0.44f);
        auto actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // Normal case.
        // Case N2: specified value is bigger than max value.
        a = FVector4(2.0f, 3.0f, 4.0f, 5.0f);
        expected = max;
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // Case N3: specified value is smaller than max value.
        a = FVector4(-2.0f, -3.0f, -4.0f, -5.0f);
        expected = min;
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // Case N4: combination case.
        a = FVector4(-2.0f, 0.5f, 4.0f, -5.0f);
        expected = FVector4(min.X, a.Y, max.Z, min.W);
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // User specified min value is bigger than max value.
        max = FVector4(0.0f, 0.1f, 0.13f, 0.14f);
        min = FVector4(1.0f, 1.1f, 1.13f, 1.14f);

        // Case W1: specified value is in the range.
        a = FVector4(0.5f, 0.3f, 0.33f, 0.44f);
        expected = min;
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // Normal case.
        // Case W2: specified value is bigger than max and min value.
        a = FVector4(2.0f, 3.0f, 4.0f, 5.0f);
        expected = min;
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");

        // Case W3: specified value is smaller than min and max value.
        a = FVector4(-2.0f, -3.0f, -4.0f, -5.0f);
        expected = min;
        actual =  a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Clamp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)

    test("Vector4LerpTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto t = 0.5f;

        auto expected = FVector4(3.0f, 4.0f, 5.0f, 6.0f);
        FVector4 actual;

        actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)
    // Lerp test with factor zero

    test("Vector4LerpTest1", []()
    {
        auto a = FVector4(FVector3(1.0f, 2.0f, 3.0f), 4.0f);
        auto b = FVector4(4.0f, 5.0f, 6.0f, 7.0f);

        auto t = 0.0f;
        auto expected = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)
    // Lerp test with factor one

    test("Vector4LerpTest2", []()
    {
        auto a = FVector4(FVector3(1.0f, 2.0f, 3.0f), 4.0f);
        auto b = FVector4(4.0f, 5.0f, 6.0f, 7.0f);

        auto t = 1.0f;
        auto expected = FVector4(4.0f, 5.0f, 6.0f, 7.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)
    // Lerp test with factor > 1

    test("Vector4LerpTest3", []()
    {
        auto a = FVector4(FVector3(0.0f, 0.0f, 0.0f), 0.0f);
        auto b = FVector4(4.0f, 5.0f, 6.0f, 7.0f);

        auto t = 2.0f;
        auto expected = FVector4(8.0f, 10.0f, 12.0f, 14.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)
    // Lerp test with factor < 0

    test("Vector4LerpTest4", []()
    {
        auto a = FVector4(FVector3(0.0f, 0.0f, 0.0f), 0.0f);
        auto b = FVector4(4.0f, 5.0f, 6.0f, 7.0f);

        auto t = -2.0f;
        auto expected = -(b * 2);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector4f, Vector4f, float)
    // Lerp test from the same point

    test("Vector4LerpTest5", []()
    {
        auto a = FVector4(4.0f, 5.0f, 6.0f, 7.0f);
        auto b = FVector4(4.0f, 5.0f, 6.0f, 7.0f);

        auto t = 0.85f;
        auto expected = a;
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Lerp did not return the expected value.");
    });

    // A test for Transform (Vector2f, FMatrix4x4)

    test("Vector4TransformTest1", []()
    {
        auto v = FVector2(1.0f, 2.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector4(10.316987f, 22.183012f, 30.3660259f, 1.0f);
        FVector4 actual;

        actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FMatrix4x4)

    test("Vector4TransformTest2", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector4(12.19198728f, 21.53349376f, 32.61602545f, 1.0f);
        FVector4 actual;

        actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "mathOps::Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FMatrix4x4)

    test("Vector4TransformVector4Test", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector4(2.19198728f, 1.53349376f, 2.61602545f, 0.0f);
        FVector4 actual;

        actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");

        // 
        v = v.setW(1);

        expected = FVector4(12.19198728f, 21.53349376f, 32.61602545f, 1.0f);
        actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FMatrix4x4)
    // Transform vector4 with zero matrix

    test("Vector4TransformVector4Test1", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);
        auto m = FMatrix4x4();
        auto expected = FVector4(0, 0, 0, 0);

        auto actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FMatrix4x4)
    // Transform vector4 with identity matrix

    test("Vector4TransformVector4Test2", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);
        auto m = FMatrix4x4::identity();
        auto expected = FVector4(1.0f, 2.0f, 3.0f, 0.0f);

        auto actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FMatrix4x4)
    // Transform Vector3f test

    test("Vector4TransformVector3Test", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FMatrix4x4::transform(FVector4(v, 1.0f), m);
        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FMatrix4x4)
    // Transform vector3 with zero matrix

    test("Vector4TransformVector3Test1", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto m = FMatrix4x4();
        auto expected = FVector4(0, 0, 0, 0);

        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FMatrix4x4)
    // Transform vector3 with identity matrix

    test("Vector4TransformVector3Test2", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto m = FMatrix4x4::identity();
        auto expected = FVector4(1.0f, 2.0f, 3.0f, 1.0f);

        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FMatrix4x4)
    // Transform Vector2f test

    test("Vector4TransformVector2Test", []()
    {
        auto v = FVector2(1.0f, 2.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FMatrix4x4::transform(FVector4(v, 0.0f, 1.0f), m);
        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FMatrix4x4)
    // Transform Vector2f with zero matrix

    test("Vector4TransformVector2Test1", []()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto m = FMatrix4x4();
        auto expected = FVector4(0, 0, 0, 0);

        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FMatrix4x4)
    // Transform vector2 with identity matrix

    test("Vector4TransformVector2Test2", []()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto m = FMatrix4x4::identity();
        auto expected = FVector4(1.0f, 2.0f, 0, 1.0f);

        auto actual = FMatrix4x4::transformToVector4(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FQuaternion)

    test("Vector4TransformVector2QuatanionTest", []()
    {
        auto v = FVector2(1.0f, 2.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));

        auto q = m.quaternion(); 

        auto expected = FMatrix4x4::transformToVector4(v, m);
        FVector4 actual;

        actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)

    test("Vector4TransformVector3Quaternion", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        auto q =  m.quaternion();

        auto expected = FMatrix4x4::transformToVector4(v, m);
        FVector4 actual;

        actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "mathOps::Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FQuaternion)

    test("Vector4TransformVector4QuaternionTest", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        auto q =  m.quaternion();

        auto expected = FMatrix4x4::transform(v, m);
        FVector4 actual;

        actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");

        // 
        v = v.setW(1);
        expected = expected.setW(1);
        actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FQuaternion)
    // Transform vector4 with zero quaternion

    test("Vector4TransformVector4QuaternionTest1", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);
        auto q = FQuaternion();
        auto expected = v;

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector4f, FQuaternion)
    // Transform vector4 with identity matrix

    test("Vector4TransformVector4QuaternionTest2", []()
    {
        auto v = FVector4(1.0f, 2.0f, 3.0f, 0.0f);
        auto q = FQuaternion::identity();
        auto expected = FVector4(1.0f, 2.0f, 3.0f, 0.0f);

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
    // Transform Vector3f test

    test("Vector4TransformVector3QuaternionTest", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        auto q =  m.quaternion();

        auto expected = FMatrix4x4::transformToVector4(v, m);
        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
    // Transform vector3 with zero quaternion

    test("Vector4TransformVector3QuaternionTest1", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto q = FQuaternion();
        auto expected = FVector4(v, 1.0f);

        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
    // Transform vector3 with identity quaternion

    test("Vector4TransformVector3QuaternionTest2", []()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto q = FQuaternion::identity();
        auto expected = FVector4(1.0f, 2.0f, 3.0f, 1.0f);

        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FQuaternion)
    // Transform Vector2f by quaternion test

    test("Vector4TransformVector2QuaternionTest", []()
    {
        auto v = FVector2(1.0f, 2.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        auto q =  m.quaternion();

        auto expected = FMatrix4x4::transformToVector4(v, m);
        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FQuaternion)
    // Transform Vector2f with zero quaternion

    test("Vector4TransformVector2QuaternionTest1", []()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto q = FQuaternion();
        auto expected = FVector4(1.0f, 2.0f, 0, 1.0f);

        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FMatrix4x4)
    // Transform vector2 with identity FQuaternion

    test("Vector4TransformVector2QuaternionTest2", []()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto q = FQuaternion::identity();
        auto expected = FVector4(1.0f, 2.0f, 0, 1.0f);

        auto actual = FQuaternion::transformToVector4(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Transform did not return the expected value.");
    });

    // A test for Normalize (Vector4f)

    test("Vector4NormalizeTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        auto expected = FVector4(
            0.1825741858350553711523232609336f,
            0.3651483716701107423046465218672f,
            0.5477225575051661134569697828008f,
            0.7302967433402214846092930437344f);
        FVector4 actual;

        actual = a.normalize();
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Normalize did not return the expected value.");
    });

    // A test for Normalize (Vector4f)
    // Normalize vector of length one

    test("Vector4NormalizeTest1", []()
    {
        auto a = FVector4(1.0f, 0.0f, 0.0f, 0.0f);

        auto expected = FVector4(1.0f, 0.0f, 0.0f, 0.0f);
        auto actual = a.normalize();
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.Normalize did not return the expected value.");
    });

    // A test for Normalize (Vector4f)
    // Normalize vector of length zero

    test("Vector4NormalizeTest2", []()
    {
        auto a = FVector4(0.0f, 0.0f, 0.0f, 0.0f);

        auto expected = FVector4(0.0f, 0.0f, 0.0f, 0.0f);
        auto actual = a.normalize();
        Assert::True(std::isnan(actual.X) && std::isnan(actual.Y) && std::isnan(actual.Z) && std::isnan(actual.W), "Vector4f.Normalize did not return the expected value.");
    });

    // A test for operator - (Vector4f)

    test("Vector4UnaryNegationTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        auto expected = FVector4(-1.0f, -2.0f, -3.0f, -4.0f);
        FVector4 actual;

        actual = -a;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator - did not return the expected value.");
    });

    // A test for operator - (Vector4f, Vector4f)

    test("Vector4SubtractionTest", []()
    {
        auto a = FVector4(1.0f, 6.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 2.0f, 3.0f, 9.0f);

        auto expected = FVector4(-4.0f, 4.0f, 0.0f, -5.0f);
        FVector4 actual;

        actual = a - b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator - did not return the expected value.");
    });

    // A test for operator * (Vector4f, float)

    test("Vector4MultiplyOperatorTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        const float factor = 2.0f;

        auto expected = FVector4(2.0f, 4.0f, 6.0f, 8.0f);
        FVector4 actual;

        actual = a * factor;
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator * did not return the expected value.");
    });

    // A test for operator * (float, Vector4f)

    test("Vector4MultiplyOperatorTest2", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        const float factor = 2.0f;
        auto expected = FVector4(2.0f, 4.0f, 6.0f, 8.0f);
        FVector4 actual;

        actual = factor * a;
        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator * did not return the expected value.");
    });

    // A test for operator * (Vector4f, Vector4f)

    test("Vector4MultiplyOperatorTest3", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FVector4(5.0f, 12.0f, 21.0f, 32.0f);
        FVector4 actual;

        actual = a * b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator * did not return the expected value.");
    });

    // A test for operator / (Vector4f, float)

    test("Vector4DivisionTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        auto div = 2.0f;

        auto expected = FVector4(0.5f, 1.0f, 1.5f, 2.0f);
        FVector4 actual;

        actual = a / div;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector4f, Vector4f)

    test("Vector4DivisionTest1", []()
    {
        auto a = FVector4(1.0f, 6.0f, 7.0f, 4.0f);
        auto b = FVector4(5.0f, 2.0f, 3.0f, 8.0f);

        auto expected = FVector4(1.0f / 5.0f, 6.0f / 2.0f, 7.0f / 3.0f, 4.0f / 8.0f);
        FVector4 actual;

        actual = a / b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector4f, Vector4f)
    // Divide by zero

    test("Vector4DivisionTest2", []()
    {
        auto a = FVector4(-2.0f, 3.0f, std::numeric_limits<float>::max(), std::numeric_limits<float>::quiet_NaN());

        auto div = 0.0f;

        auto actual = a / div;

        Assert::True(std::isinf(actual.X) && actual.X < 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y > 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Z) && actual.Z > 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isnan(actual.W), "Vector4f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector4f, Vector4f)
    // Divide by zero

    test("Vector4DivisionTest3", []()
    {
        auto a = FVector4(0.047f, -3.0f, -std::numeric_limits<float>::infinity(), std::numeric_limits<float>::lowest());
        auto b = FVector4();

        auto actual = a / b;

        Assert::True(std::isinf(actual.X) && actual.X > 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y < 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Z) && actual.Z < 0, "Vector4f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.W) && actual.W < 0, "Vector4f.operator / did not return the expected value.");
        //    Assert::True(float.IsNegativeInfinity(actual.W), "Vector4f.operator / did not return the expected value.");
    });

    // A test for operator + (Vector4f, Vector4f)

    test("Vector4AdditionTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FVector4(6.0f, 8.0f, 10.0f, 12.0f);
        FVector4 actual;

        actual = a + b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector4f.operator + did not return the expected value.");
    });


    test("OperatorAddTest", []()
    {
        auto v1 = FVector4(2.5f, 2.0f, 3.0f, 3.3f);
        auto v2 = FVector4(5.5f, 4.5f, 6.5f, 7.5f);

        auto v3 = v1 + v2;
        auto v5 = FVector4(-1.0f, 0.0f, 0.0f, std::numeric_limits<float>::quiet_NaN());
        auto v4 = v1 + v5;
        Assert::AreEqual(8.0f, v3.X);
        Assert::AreEqual(6.5f, v3.Y);
        Assert::AreEqual(9.5f, v3.Z);
        Assert::AreEqual(10.8f, v3.W);
        Assert::AreEqual(1.5f, v4.X);
        Assert::AreEqual(2.0f, v4.Y);
        Assert::AreEqual(3.0f, v4.Z);
        Assert::True(std::isnan(v4.W));
    });

    // A test for Vector4f (float, float, float, float)

    test("Vector4ConstructorTest", []()
    {
        auto x = 1.0f;
        auto y = 2.0f;
        auto z = 3.0f;
        auto w = 4.0f;

        auto target = FVector4(x, y, z, w);

        Assert::True(MathHelper::Equal(target.X, x) && MathHelper::Equal(target.Y, y) && MathHelper::Equal(target.Z, z) && MathHelper::Equal(target.W, w),
            "Vector4f constructor(x,y,z,w) did not return the expected value.");
    });

    // A test for Vector4f (Vector2f, float, float)

    test("Vector4ConstructorTest1", []()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto z = 3.0f;
        auto w = 4.0f;

        auto target = FVector4(a, z, w);
        Assert::True(MathHelper::Equal(target.X, a.X) && MathHelper::Equal(target.Y, a.Y) && MathHelper::Equal(target.Z, z) && MathHelper::Equal(target.W, w),
            "Vector4f constructor(Vector2f,z,w) did not return the expected value.");
    });

    // A test for Vector4f (Vector3f, float)

    test("Vector4ConstructorTest2", []()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto w = 4.0f;

        auto target = FVector4(a, w);

        Assert::True(MathHelper::Equal(target.X, a.X) && MathHelper::Equal(target.Y, a.Y) && MathHelper::Equal(target.Z, a.Z) && MathHelper::Equal(target.W, w),
            "Vector4f constructor(Vector3f,w) did not return the expected value.");
    });

    // A test for Vector4f ()
    // Constructor with no parameter

    test("Vector4ConstructorTest4", []()
    {
        auto a = FVector4();

        Assert::AreEqual(a.X, 0.0f);
        Assert::AreEqual(a.Y, 0.0f);
        Assert::AreEqual(a.Z, 0.0f);
        Assert::AreEqual(a.W, 0.0f);
    });

    // A test for Vector4f ()
    // Constructor with special floating values

    test("Vector4ConstructorTest5", []()
    {
        auto target = FVector4(std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::max(), std::numeric_limits<float>::infinity(), std::numeric_limits<float>::epsilon());

        Assert::True(std::isnan(target.X), "Vector4f.constructor (float, float, float, float) did not return the expected value.");
        Assert::True((std::numeric_limits<float>::max() == target.Y), "Vector4f.constructor (float, float, float, float) did not return the expected value.");
        Assert::True(std::isinf(target.Z) && target.Z > 0, "Vector4f.constructor (float, float, float, float) did not return the expected value.");
        Assert::True((std::numeric_limits<float>::epsilon() == target.W), "Vector4f.constructor (float, float, float, float) did not return the expected value.");
    });

    // A test for Add (Vector4f, Vector4f)

    test("Vector4AddTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FVector4(6.0f, 8.0f, 10.0f, 12.0f);
        FVector4 actual;

        actual = a.add(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Divide (Vector4f, float)

    test("Vector4DivideTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto div = 2.0f;
        auto expected = FVector4(0.5f, 1.0f, 1.5f, 2.0f);
        Assert::AreEqual(expected, a / div);
    });

    // A test for Divide (Vector4f, Vector4f)

    test("Vector4DivideTest1", []()
    {
        auto a = FVector4(1.0f, 6.0f, 7.0f, 4.0f);
        auto b = FVector4(5.0f, 2.0f, 3.0f, 8.0f);

        auto expected = FVector4(1.0f / 5.0f, 6.0f / 2.0f, 7.0f / 3.0f, 4.0f / 8.0f);
        FVector4 actual;

        actual = a.divide(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Equals (object)

    test("Vector4EqualsTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto obj = b;

        bool expected = true;
        bool actual = a == obj;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10);
        obj = b;
        expected = false;
        actual = a == obj;
        Assert::AreEqual(expected, actual);

        //// case 3: compare between different types.
        //obj = FQuaternion();
        //expected = false;
        //actual = a == obj;
        //Assert::AreEqual(expected, actual);

        //// case 3: compare against null.
        ////obj = null;
        //expected = false;
        //actual = a == obj;
        Assert::AreEqual(expected, actual);
    });

    // A test for Multiply (float, Vector4f)

    test("Vector4MultiplyTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        const float factor = 2.0f;
        auto expected = FVector4(2.0f, 4.0f, 6.0f, 8.0f);
        Assert::AreEqual(expected, factor * a);
    });

    // A test for Multiply (Vector4f, float)

    test("Vector4MultiplyTest2", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        const float factor = 2.0f;
        auto expected = FVector4(2.0f, 4.0f, 6.0f, 8.0f);
        Assert::AreEqual(expected, a * factor);
    });

    // A test for Multiply (Vector4f, Vector4f)

    test("Vector4MultiplyTest3", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FVector4(5.0f, 12.0f, 21.0f, 32.0f);
        FVector4 actual;

        actual = a.multiply(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Negate (Vector4f)

    test("Vector4NegateTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        auto expected = FVector4(-1.0f, -2.0f, -3.0f, -4.0f);
        FVector4 actual;

        actual = a.negate();
        Assert::AreEqual(expected, actual);
    });

    // A test for operator != (Vector4f, Vector4f)

    test("Vector4InequalityTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = false;
        auto actual = a != b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10);
        expected = true;
        actual = a != b;
        Assert::AreEqual(expected, actual);
    });

    // A test for operator == (Vector4f, Vector4f)

    test("Vector4EqualityTest", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10);
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for Subtract (Vector4f, Vector4f)

    test("Vector4SubtractTest", []()
    {
        auto a = FVector4(1.0f, 6.0f, 3.0f, 4.0f);
        auto b = FVector4(5.0f, 2.0f, 3.0f, 9.0f);

        auto expected = FVector4(-4.0f, 4.0f, 0.0f, -5.0f);
        FVector4 actual;

        actual = a.subtract(b);

        Assert::AreEqual(expected, actual);
    });

    // A test for UnitW

    test("Vector4UnitWTest", []()
    {
        auto val = FVector4(0.0f, 0.0f, 0.0f, 1.0f);
        Assert::AreEqual(val, FVector4::unitW());
    });

    // A test for UnitX

    test("Vector4UnitXTest", []()
    {
        auto val = FVector4(1.0f, 0.0f, 0.0f, 0.0f);
        Assert::AreEqual(val, FVector4::unitX());
    });

    // A test for UnitY

    test("Vector4UnitYTest", []()
    {
        auto val = FVector4(0.0f, 1.0f, 0.0f, 0.0f);
        Assert::AreEqual(val, FVector4::unitY());
    });

    // A test for UnitZ

    test("Vector4UnitZTest", []()
    {
        auto val = FVector4(0.0f, 0.0f, 1.0f, 0.0f);
        Assert::AreEqual(val, FVector4::unitZ());
    });

    // A test for One

    test("Vector4OneTest", []()
    {
        auto val = FVector4(1.0f, 1.0f, 1.0f, 1.0f);
        Assert::AreEqual(val, FVector4::one());
    });

    // A test for Zero

    test("Vector4ZeroTest", []()
    {
        auto val = FVector4(0.0f, 0.0f, 0.0f, 0.0f);
        Assert::AreEqual(val, FVector4::zero());
    });

    // A test for Equals (Vector4f)

    test("Vector4EqualsTest1", []()
    {
        auto a = FVector4(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FVector4(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        Assert::True(a == b);

        // case 2: compare between different values
        b = b.setX(10);
        Assert::False(a == b);
    });

    // A test for Vector4f (float)

    test("Vector4ConstructorTest6", []()
    {
        auto value = 1.0f;
        auto target = FVector4(value);

        auto expected = FVector4(value, value, value, value);
        Assert::AreEqual(expected, target);

        value = 2.0f;
        target = FVector4(value);
        expected = FVector4(value, value, value, value);
        Assert::AreEqual(expected, target);
    });

    // A test for Vector4f comparison involving NaN values

    test("Vector4EqualsNanTest", []()
    {
        auto a = FVector4(std::numeric_limits<float>::quiet_NaN(), 0, 0, 0);
        auto b = FVector4(0, std::numeric_limits<float>::quiet_NaN(), 0, 0);
        auto c = FVector4(0, 0, std::numeric_limits<float>::quiet_NaN(), 0);
        auto d = FVector4(0, 0, 0, std::numeric_limits<float>::quiet_NaN());

        Assert::False(a == FVector4::zero());
        Assert::False(b == FVector4::zero());
        Assert::False(c == FVector4::zero());
        Assert::False(d == FVector4::zero());

        Assert::True(a != FVector4::zero());
        Assert::True(b != FVector4::zero());
        Assert::True(c != FVector4::zero());
        Assert::True(d != FVector4::zero());

        Assert::False(a == FVector4::zero());
        Assert::False(b == FVector4::zero());
        Assert::False(c == FVector4::zero());
        Assert::False(d == FVector4::zero());

        // Counterintuitive result - IEEE rules for NaN comparison are weird!
        Assert::False(a == (a));
        Assert::False(b == (b));
        Assert::False(c == (c));
        Assert::False(d == (d));
    });


    test("Vector4AbsTest", []()
    {
        auto v1 = FVector4(-2.5f, 2.0f, 3.0f, 3.3f);
        auto v3 = FVector4(std::numeric_limits<float>::infinity(), 0.0f, -std::numeric_limits<float>::infinity(), std::numeric_limits<float>::quiet_NaN()).abs();
        auto v = v1.abs();
        Assert::AreEqual(2.5f, v.X);
        Assert::AreEqual(2.0f, v.Y);
        Assert::AreEqual(3.0f, v.Z);
        Assert::AreEqual(3.3f, v.W);
        Assert::True(std::isinf(v3.X));
        Assert::AreEqual(0.0f, v3.Y);
        Assert::True(std::isinf(v3.Z));
        Assert::True(std::isnan(v3.W));
    });


    test("Vector4SqrtTest", []()
    {
        auto v1 = FVector4(-2.5f, 2.0f, 3.0f, 3.3f);
        auto v2 = FVector4(5.5f, 4.5f, 6.5f, 7.5f);
        Assert::AreEqual(2, (int)v2.squareRoot().X);
        Assert::AreEqual(2, (int)v2.squareRoot().Y);
        Assert::AreEqual(2, (int)v2.squareRoot().Z);
        Assert::AreEqual(2, (int)v2.squareRoot().W);
        Assert::True(std::isnan(v1.squareRoot().X));
    });

#pragma endregion

 
    return 0;
}