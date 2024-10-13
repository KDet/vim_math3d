#include "test_vim_math3d.h"

int main()
{
#pragma region Vector3Tests
    std::cout << "FVector3 Tests" << std::endl;

    test("Vector3MarshalSizeTest", []()
    {
        auto v = FVector3();

        Assert::True(12 == sizeof(FVector3));
        Assert::True(12 == sizeof(struct FVector3));
        Assert::True(12 == sizeof(v));
    });

    test("Vector3hashTest", [] ()
    {
        auto v1 = FVector3(2.0f, 3.0f, 3.3f);
        auto v2 = FVector3(2.0f, 3.0f, 3.3f);
        auto v3 = FVector3(2.0f, 3.0f, 3.3f);
        auto v5 = FVector3(3.0f, 2.0f, 3.3f);
        Assert::AreEqual(v1.hash(), v1.hash());
        Assert::AreEqual(v1.hash(), v2.hash());
        Assert::AreNotEqual(v1.hash(), v5.hash());
        Assert::AreEqual(v1.hash(), v3.hash());
        auto v4 = FVector3(0.0f, 0.0f, 0.0f);
        auto v6 = FVector3(1.0f, 0.0f, 0.0f);
        auto v7 = FVector3(0.0f, 1.0f, 0.0f);
        auto v8 = FVector3(1.0f, 1.0f, 1.0f);
        auto v9 = FVector3(1.0f, 1.0f, 0.0f);
        Assert::AreNotEqual(v4.hash(), v6.hash());
        Assert::AreNotEqual(v4.hash(), v7.hash());
        Assert::AreNotEqual(v4.hash(), v8.hash());
        Assert::AreNotEqual(v7.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v9.hash());
        Assert::AreNotEqual(v7.hash(), v9.hash());
    });

    // A test for Cross (Vector3f, Vector3f)
        
    test("Vector3CrossTest", [] ()
    {
        auto a = FVector3(1.0f, 0.0f, 0.0f);
        auto b = FVector3(0.0f, 1.0f, 0.0f);

        auto expected = FVector3(0.0f, 0.0f, 1.0f);
        FVector3 actual;

        actual = a.cross(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Cross did not return the expected value.");
    });

    // A test for Cross (Vector3f, Vector3f)
    // Cross test of the same vector
        
    test("Vector3CrossTest1", [] ()
    {
        auto a = FVector3(0.0f, 1.0f, 0.0f);
        auto b = FVector3(0.0f, 1.0f, 0.0f);

        auto expected = FVector3(0.0f, 0.0f, 0.0f);
        auto actual = a.cross(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Cross did not return the expected value.");
    });

    // A test for Distance (Vector3f, Vector3f)
        
    test("Vector3DistanceTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto expected = (float)std::sqrt(27);
        float actual;

        actual = a.distance(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Distance did not return the expected value.");
    });

    // A test for Distance (Vector3f, Vector3f)
    // Distance from the same point
        
    test("Vector3DistanceTest1", [] ()
    {
        auto a = FVector3(1.051f, 2.05f, 3.478f);
        auto b = FVector3(1.051f, 2.05f, 3.478f);

        auto actual = a.distance(b);
        Assert::AreEqual(0.0f, actual);
    });

    // A test for DistanceSquared (Vector3f, Vector3f)
        
    test("Vector3DistanceSquaredTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto expected = 27.0f;
        float actual;

        actual = a.distanceSquared(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.DistanceSquared did not return the expected value.");
    });

    // A test for Dot (Vector3f, Vector3f)
        
    test("Vector3DotTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto expected = 32.0f;
        float actual;

        actual = a.dot(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Dot did not return the expected value.");
    });

    // A test for Dot (Vector3f, Vector3f)
    // Dot test for perpendicular vector
        
    test("Vector3DotTest1", [] ()
    {
        auto a = FVector3(1.55f, 1.55f, 1);
        auto b = FVector3(2.5f, 3, 1.5f);
        auto c = a.cross(b);

        auto expected = 0.0f;
        auto actual1 = a.dot(c);
        auto actual2 = b.dot(c);
        Assert::True(MathHelper::Equal(expected, actual1), "Vector3f.Dot did not return the expected value.");
        Assert::True(MathHelper::Equal(expected, actual2), "Vector3f.Dot did not return the expected value.");
    });

    // A test for Length ()
        
    test("Vector3LengthTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);

        auto z = 3.0f;

        auto target = FVector3(a, z);

        auto expected = (float)std::sqrt(14.0f);
        float actual;

        actual = target.length();
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Length did not return the expected value.");
    });

    // A test for Length ()
    // Length test where length is zero
        
    test("Vector3LengthTest1", [] ()
    {
        auto target = FVector3();

        auto expected = 0.0f;
        auto actual = target.length();
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Length did not return the expected value.");
    });

    // A test for LengthSquared ()
        
    test("Vector3LengthSquaredTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);

        auto z = 3.0f;

        auto target = FVector3(a, z);

        auto expected = 14.0f;
        float actual;

        actual = target.lengthSquared();
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.LengthSquared did not return the expected value.");
    });

    // A test for Min (Vector3f, Vector3f)
        
    test("Vector3MinTest", [] ()
    {
        auto a = FVector3(-1.0f, 4.0f, -3.0f);
        auto b = FVector3(2.0f, 1.0f, -1.0f);

        auto expected = FVector3(-1.0f, 1.0f, -3.0f);
        FVector3 actual;
        actual = a.minimum(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Min did not return the expected value.");
    });

    // A test for Max (Vector3f, Vector3f)
        
    test("Vector3MaxTest", [] ()
    {
        auto a = FVector3(-1.0f, 4.0f, -3.0f);
        auto b = FVector3(2.0f, 1.0f, -1.0f);

        auto expected = FVector3(2.0f, 4.0f, -1.0f);
        FVector3 actual;
        actual = a.maximum(b);
        Assert::True(MathHelper::Equal(expected, actual), "mathOps::Max did not return the expected value.");
    });

        
    test("Vector3MinMaxCodeCoverageTest", [] ()
    {
        auto min = FVector3::zero();
        auto max = FVector3::one();
        FVector3 actual;

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

    // A test for Lerp (Vector3f, Vector3f, float)
        
    test("Vector3LerpTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto t = 0.5f;

        auto expected = FVector3(2.5f, 3.5f, 4.5f);
        FVector3 actual;

        actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector3f, Vector3f, float)
    // Lerp test with factor zero
        
    test("Vector3LerpTest1", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto t = 0.0f;
        auto expected = FVector3(1.0f, 2.0f, 3.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector3f, Vector3f, float)
    // Lerp test with factor one
        
    test("Vector3LerpTest2", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto t = 1.0f;
        auto expected = FVector3(4.0f, 5.0f, 6.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector3f, Vector3f, float)
    // Lerp test with factor > 1
        
    test("Vector3LerpTest3", [] ()
    {
        auto a = FVector3(0.0f, 0.0f, 0.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto t = 2.0f;
        auto expected = FVector3(8.0f, 10.0f, 12.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector3f, Vector3f, float)
    // Lerp test with factor < 0
        
    test("Vector3LerpTest4", [] ()
    {
        auto a = FVector3(0.0f, 0.0f, 0.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto t = -2.0f;
        auto expected = FVector3(-8.0f, -10.0f, -12.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector3f, Vector3f, float)
    // Lerp test from the same point
        
    test("Vector3LerpTest5", [] ()
    {
        auto a = FVector3(1.68f, 2.34f, 5.43f);
        auto b = a;

        auto t = 0.18f;
        auto expected = FVector3(1.68f, 2.34f, 5.43f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Lerp did not return the expected value.");
    });

    // A test for Reflect (Vector3f, Vector3f)
        
    test("Vector3ReflectTest", [] ()
    {
        auto a = FVector3(1.0f, 1.0f, 1.0f).normalize();

        // Reflect on XZ plane.
        auto n = FVector3(0.0f, 1.0f, 0.0f);
        auto expected = FVector3(a.X, -a.Y, a.Z);
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");

        // Reflect on XY plane.
        n = FVector3(0.0f, 0.0f, 1.0f);
        expected = FVector3(a.X, a.Y, -a.Z);
        actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");

        // Reflect on YZ plane.
        n = FVector3(1.0f, 0.0f, 0.0f);
        expected = FVector3(-a.X, a.Y, a.Z);
        actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");
    });

    // A test for Reflect (Vector3f, Vector3f)
    // Reflection when normal and source are the same
        
    test("Vector3ReflectTest1", [] ()
    {
        auto n = FVector3(0.45f, 1.28f, 0.86f);
        n = n.normalize();
        auto a = n;

        auto expected = -n;
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");
    });

    // A test for Reflect (Vector3f, Vector3f)
    // Reflection when normal and source are negation
        
    test("Vector3ReflectTest2", [] ()
    {
        auto n = FVector3(0.45f, 1.28f, 0.86f);
        n = n.normalize();
        auto a = -n;

        auto expected = n;
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");
    });

    // A test for Reflect (Vector3f, Vector3f)
    // Reflection when normal and source are perpendicular (a dot n = 0)
        
    test("Vector3ReflectTest3", [] ()
    {
        auto n = FVector3(0.45f, 1.28f, 0.86f);
        auto temp = FVector3(1.28f, 0.45f, 0.01f);
        // find a perpendicular vector of n
        auto a = temp.cross(n);

        auto expected = a;
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Reflect did not return the expected value.");
    });

    // A test for Transform(Vector3f, FMatrix4x4)
        
    test("Vector3TransformTest", [] ()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector3(12.191987f, 21.533493f, 32.616024f);
        FVector3 actual;

        actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Transform did not return the expected value.");
    });

    // A test for Clamp (Vector3f, Vector3f, Vector3f)
        
    test("Vector3ClampTest", [] ()
    {
        auto a = FVector3(0.5f, 0.3f, 0.33f);
        auto min = FVector3(0.0f, 0.1f, 0.13f);
        auto max = FVector3(1.0f, 1.1f, 1.13f);

        // Normal case.
        // Case N1: specified value is in the range.
        auto expected = FVector3(0.5f, 0.3f, 0.33f);
        auto actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // Normal case.
        // Case N2: specified value is bigger than max value.
        a = FVector3(2.0f, 3.0f, 4.0f);
        expected = max;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // Case N3: specified value is smaller than max value.
        a = FVector3(-2.0f, -3.0f, -4.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // Case N4: combination case.
        a = FVector3(-2.0f, 0.5f, 4.0f);
        expected = FVector3(min.X, a.Y, max.Z);
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // User specified min value is bigger than max value.
        max = FVector3(0.0f, 0.1f, 0.13f);
        min = FVector3(1.0f, 1.1f, 1.13f);

        // Case W1: specified value is in the range.
        a = FVector3(0.5f, 0.3f, 0.33f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // Normal case.
        // Case W2: specified value is bigger than max and min value.
        a = FVector3(2.0f, 3.0f, 4.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");

        // Case W3: specified value is smaller than min and max value.
        a = FVector3(-2.0f, -3.0f, -4.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Clamp did not return the expected value.");
    });

    // A test for TransformNormal (Vector3f, FMatrix4x4)
        
    test("Vector3TransformNormalTest", [] ()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector3(2.19198728f, 1.53349364f, 2.61602545f);
        FVector3 actual;

        actual = FMatrix4x4::transformNormal(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.TransformNormal did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
        
    test("Vector3TransformByQuaternionTest", [] ()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        auto q = m.quaternion();

        auto expected = FMatrix4x4::transform(v, m);
        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
    // Transform vector3 with zero quaternion
        
    test("Vector3TransformByQuaternionTest1", [] ()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto q = FQuaternion();
        auto expected = v;

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector3f, FQuaternion)
    // Transform vector3 with identity quaternion
        
    test("Vector3TransformByQuaternionTest2", [] ()
    {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto q = FQuaternion::identity();
        auto expected = v;

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Transform did not return the expected value.");
    });

    // A test for Normalize (Vector3f)
        
    test("Vector3NormalizeTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto expected = FVector3(
            0.26726124191242438468455348087975f,
            0.53452248382484876936910696175951f,
            0.80178372573727315405366044263926f);
        FVector3 actual;

        actual = a.normalize();
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Normalize did not return the expected value.");
    });

    // A test for Normalize (Vector3f)
    // Normalize vector of length one
        
    test("Vector3NormalizeTest1", [] ()
    {
        auto a = FVector3(1.0f, 0.0f, 0.0f);

        auto expected = FVector3(1.0f, 0.0f, 0.0f);
        auto actual = a.normalize();
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Normalize did not return the expected value.");
    });

    // A test for Normalize (Vector3f)
    // Normalize vector of length zero
        
    test("Vector3NormalizeTest2", [] ()
    {
        auto a = FVector3(0.0f, 0.0f, 0.0f);

        auto expected = FVector3(0.0f, 0.0f, 0.0f);
        auto actual = a.normalize();
        Assert::True(std::isnan(actual.X) && std::isnan(actual.Y) && std::isnan(actual.Z), "Vector3f.Normalize did not return the expected value.");
    });

    // A test for operator - (Vector3f)
        
    test("Vector3UnaryNegationTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto expected = FVector3(-1.0f, -2.0f, -3.0f);
        FVector3 actual;

        actual = -a;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator - did not return the expected value.");
    });

        
    test("Vector3UnaryNegationTest1", []() 
    {
        auto a = -FVector3(std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::infinity(), -std::numeric_limits<float>::infinity());
        auto b = -FVector3(0.0f, 0.0f, 0.0f);
        Assert::True(std::isnan(a.X));
        Assert::AreEqual(-std::numeric_limits<float>::infinity(), a.Y);
        Assert::AreEqual(std::numeric_limits<float>::infinity(), a.Z);
        Assert::AreEqual(0.0f, b.X);
        Assert::AreEqual(0.0f, b.Y);
        Assert::AreEqual(0.0f, b.Z);
    });

    // A test for operator - (Vector3f, Vector3f)
        
    test("Vector3SubtractionTest", [] ()
    {
        auto a = FVector3(4.0f, 2.0f, 3.0f);

        auto b = FVector3(1.0f, 5.0f, 7.0f);

        auto expected = FVector3(3.0f, -3.0f, -4.0f);
        FVector3 actual;

        actual = a - b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator - did not return the expected value.");
    });

    // A test for operator * (Vector3f, float)
        
    test("Vector3MultiplyOperatorTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto factor = 2.0f;

        auto expected = FVector3(2.0f, 4.0f, 6.0f);
        FVector3 actual;

        actual = a * factor;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator * did not return the expected value.");
    });

    // A test for operator * (float, Vector3f)
        
    test("Vector3MultiplyOperatorTest2", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        const float factor = 2.0f;

        auto expected = FVector3(2.0f, 4.0f, 6.0f);
        FVector3 actual;

        actual = factor * a;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator * did not return the expected value.");
    });

    // A test for operator * (Vector3f, Vector3f)
        
    test("Vector3MultiplyOperatorTest3", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto expected = FVector3(4.0f, 10.0f, 18.0f);
        FVector3 actual;

        actual = a * b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator * did not return the expected value.");
    });

    // A test for operator / (Vector3f, float)
        
    test("Vector3DivisionTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto div = 2.0f;

        auto expected = FVector3(0.5f, 1.0f, 1.5f);
        FVector3 actual;

        actual = a / div;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector3f, Vector3f)
        
    test("Vector3DivisionTest1", [] ()
    {
        auto a = FVector3(4.0f, 2.0f, 3.0f);

        auto b = FVector3(1.0f, 5.0f, 6.0f);

        auto expected = FVector3(4.0f, 0.4f, 0.5f);
        FVector3 actual;

        actual = a / b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector3f, Vector3f)
    // Divide by zero
        
    test("Vector3DivisionTest2", [] ()
    {
        auto a = FVector3(-2.0f, 3.0f, std::numeric_limits<float>::max());

        auto div = 0.0f;

        auto actual = a / div;

        Assert::True(std::isinf(actual.X) && actual.X < 0, "Vector3f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y > 0, "Vector3f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Z) && actual.Z > 0, "Vector3f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector3f, Vector3f)
    // Divide by zero
        
    test("Vector3DivisionTest3", [] ()
    {
        auto a = FVector3(0.047f, -3.0f, -std::numeric_limits<float>::infinity());
        auto b = FVector3();

        auto actual = a / b;

        Assert::True(std::isinf(actual.X) && actual.X > 0, "Vector3f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y < 0, "Vector3f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Z) && actual.Z < 0, "Vector3f.operator / did not return the expected value.");
    });

    // A test for operator + (Vector3f, Vector3f)
        
    test("Vector3AdditionTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(4.0f, 5.0f, 6.0f);

        auto expected = FVector3(5.0f, 7.0f, 9.0f);
        FVector3 actual;

        actual = a + b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.operator + did not return the expected value.");
    });

    // A test for Vector3f (float, float, float)
        
    test("Vector3ConstructorTest", [] ()
    {
        auto x = 1.0f;
        auto y = 2.0f;
        auto z = 3.0f;

        auto target = FVector3(x, y, z);
        Assert::True(MathHelper::Equal(target.X, x) && MathHelper::Equal(target.Y, y) && MathHelper::Equal(target.Z, z), "Vector3f.constructor (x,y,z) did not return the expected value.");
    });

    // A test for Vector3f (Vector2f, float)
        
    test("Vector3ConstructorTest1", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);

        auto z = 3.0f;

        auto target = FVector3(a, z);
        Assert::True(MathHelper::Equal(target.X, a.X) && MathHelper::Equal(target.Y, a.Y) && MathHelper::Equal(target.Z, z), "Vector3f.constructor (Vector2f,z) did not return the expected value.");
    });

    // A test for Vector3f ()
    // Constructor with no parameter
        
    test("Vector3ConstructorTest3", [] ()
    {
        auto a = FVector3();

        Assert::AreEqual(a.X, 0.0f);
        Assert::AreEqual(a.Y, 0.0f);
        Assert::AreEqual(a.Z, 0.0f);
    });

    // A test for Vector2f (float, float)
    // Constructor with special floating values
        
    test("Vector3ConstructorTest4", [] ()
    {
        auto target = FVector3(std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::max(), std::numeric_limits<float>::infinity());

        Assert::True(std::isnan(target.X), "Vector3f.constructor (Vector3f) did not return the expected value.");
        Assert::True((std::numeric_limits<float>::max() == target.Y), "Vector3f.constructor (Vector3f) did not return the expected value.");
        Assert::True(std::isinf(target.Z) && target.Z > 0, "Vector3f.constructor (Vector3f) did not return the expected value.");
    });

    // A test for Add (Vector3f, Vector3f)
        
    test("Vector3AddTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(5.0f, 6.0f, 7.0f);

        auto expected = FVector3(6.0f, 8.0f, 10.0f);
        FVector3 actual;

        actual = a.add(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Divide (Vector3f, float)
        
    test("Vector3DivideTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto div = 2.0f;
        auto expected = FVector3(0.5f, 1.0f, 1.5f);
        Assert::AreEqual(expected, a / div);
    });

    // A test for Divide (Vector3f, Vector3f)
        
    test("Vector3DivideTest1", [] ()
    {
        auto a = FVector3(1.0f, 6.0f, 7.0f);
        auto b = FVector3(5.0f, 2.0f, 3.0f);

        auto expected = FVector3(1.0f / 5.0f, 6.0f / 2.0f, 7.0f / 3.0f);
        FVector3 actual;

        actual = a.divide(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Equals (object)
        
    test("Vector3EqualsTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(1.0f, 2.0f, 3.0f);

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

    // A test for Multiply (Vector3f, float)
        
    test("Vector3MultiplyTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        const float factor = 2.0f;
        auto expected = FVector3(2.0f, 4.0f, 6.0f);
        Assert::AreEqual(expected, a * factor);
    });

    // A test for Multiply (float, Vector3f)
        
    test("Vector3MultiplyTest2", []()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        const float factor = 2.0f;
        auto expected = FVector3(2.0f, 4.0f, 6.0f);
        Assert::AreEqual(expected, factor * a);
    });

    // A test for Multiply (Vector3f, Vector3f)
        
    test("Vector3MultiplyTest3", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(5.0f, 6.0f, 7.0f);

        auto expected = FVector3(5.0f, 12.0f, 21.0f);
        FVector3 actual;

        actual = a.multiply(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Negate (Vector3f)
        
    test("Vector3NegateTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);

        auto expected = FVector3(-1.0f, -2.0f, -3.0f);
        FVector3 actual;

        actual = a.negate();
        Assert::AreEqual(expected, actual);
    });

    // A test for operator != (Vector3f, Vector3f)
        
    test("Vector3InequalityTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(1.0f, 2.0f, 3.0f);

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

    // A test for operator == (Vector3f, Vector3f)
        
    test("Vector3EqualityTest", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(1.0f, 2.0f, 3.0f);

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

    // A test for Subtract (Vector3f, Vector3f)
        
    test("Vector3SubtractTest", [] ()
    {
        auto a = FVector3(1.0f, 6.0f, 3.0f);
        auto b = FVector3(5.0f, 2.0f, 3.0f);

        auto expected = FVector3(-4.0f, 4.0f, 0.0f);
        FVector3 actual;

        actual = a.subtract(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for One
        
    test("Vector3OneTest", [] ()
    {
        auto val = FVector3(1.0f, 1.0f, 1.0f);
        Assert::AreEqual(val, FVector3::one());
    });

    // A test for UnitX
        
    test("Vector3UnitXTest", [] ()
    {
        auto val = FVector3(1.0f, 0.0f, 0.0f);
        Assert::AreEqual(val, FVector3::unitX());
    });

    // A test for UnitY
        
    test("Vector3UnitYTest", [] ()
    {
        auto val = FVector3(0.0f, 1.0f, 0.0f);
        Assert::AreEqual(val, FVector3::unitY());
    });

    // A test for UnitZ
        
    test("Vector3UnitZTest", [] ()
    {
        auto val = FVector3(0.0f, 0.0f, 1.0f);
        Assert::AreEqual(val, FVector3::unitZ());
    });

    // A test for Zero
        
    test("Vector3ZeroTest", [] ()
    {
        auto val = FVector3(0.0f, 0.0f, 0.0f);
        Assert::AreEqual(val, FVector3::zero());
    });

    // A test for Equals (Vector3f)
        
    test("Vector3EqualsTest1", [] ()
    {
        auto a = FVector3(1.0f, 2.0f, 3.0f);
        auto b = FVector3(1.0f, 2.0f, 3.0f);

        // case 1: compare between same values
        bool expected = true;
        bool actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10);
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for Vector3f (float)
        
    test("Vector3ConstructorTest5", [] ()
    {
        auto value = 1.0f;
        auto target = FVector3(value);

        auto expected = FVector3(value, value, value);
        Assert::AreEqual(expected, target);

        value = 2.0f;
        target = FVector3(value);
        expected = FVector3(value, value, value);
        Assert::AreEqual(expected, target);
    });

    // A test for Vector3f comparison involving NaN values
        
    test("Vector3EqualsNanTest", [] ()
    {
        auto a = FVector3(std::numeric_limits<float>::quiet_NaN(), 0, 0);
        auto b = FVector3(0, std::numeric_limits<float>::quiet_NaN(), 0);
        auto c = FVector3(0, 0, std::numeric_limits<float>::quiet_NaN());

        Assert::False(a == FVector3::zero());
        Assert::False(b == FVector3::zero());
        Assert::False(c == FVector3::zero());

        Assert::True(a != FVector3::zero());
        Assert::True(b != FVector3::zero());
        Assert::True(c != FVector3::zero());

        Assert::False(a == FVector3::zero());
        Assert::False(b == FVector3::zero());
        Assert::False(c == FVector3::zero());

        // Counterintuitive result - IEEE rules for NaN comparison are weird!
        Assert::False(a == a);
        Assert::False(b == b);
        Assert::False(c == c);
    });

        
    test("Vector3AbsTest", [] ()
    {
        auto v1 = FVector3(-2.5f, 2.0f, 0.5f);
        auto v3 = FVector3(0.0f, -std::numeric_limits<float>::infinity(), std::numeric_limits<float>::quiet_NaN()).abs();
        auto v = v1.abs();
        Assert::True(2.5f == v.X);
        Assert::True(2.0f == v.Y);
        Assert::True(0.5f == v.Z);
        Assert::True(0.0f == v3.X);
        Assert::True(std::isinf(v3.Y));
        Assert::True(std::isnan(v3.Z));
    });

        
    test("Vector3SqrtTest", [] ()
    {
        auto a = FVector3(-2.5f, 2.0f, 0.5f);
        auto b = FVector3(5.5f, 4.5f, 16.5f);
        Assert::True(2 == (int)b.squareRoot().X);
        Assert::True(2 == (int)b.squareRoot().Y);
        Assert::True(4 == (int)b.squareRoot().Z);
        Assert::True(std::isnan(a.squareRoot().X));
    });


#pragma endregion

    return 0;
}