#include "test_vim_math3d.h"

int main()
{
#pragma region Vector2Tests
    std::cout << "FVector2 Tests" << std::endl;
    
    test("Vector2MarshalSizeTest", []()
    {
        auto v = FVector2();

        Assert::True(8 == sizeof(FVector2));
        Assert::True(8 == sizeof(struct FVector2));
        Assert::True(8 == sizeof(v));
    });

    test("Vector2hashTest", [] ()
    {
        auto v1 = FVector2(2.0f, 3.0f);
        auto v2 = FVector2(2.0f, 3.0f);
        auto v3 = FVector2(3.0f, 2.0f);
        Assert::AreEqual(v1.hash(), v1.hash());
        Assert::AreEqual(v1.hash(), v2.hash());
        Assert::AreNotEqual(v1.hash(), v3.hash());
        auto v4 = FVector2(0.0f, 0.0f);
        auto v6 = FVector2(1.0f, 0.0f);
        auto v7 = FVector2(0.0f, 1.0f);
        auto v8 = FVector2(1.0f, 1.0f);
        Assert::AreNotEqual(v4.hash(), v6.hash());
        Assert::AreNotEqual(v4.hash(), v7.hash());
        Assert::AreNotEqual(v4.hash(), v8.hash());
        Assert::AreNotEqual(v7.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v6.hash());
        Assert::AreNotEqual(v8.hash(), v7.hash());
    });

    // A test for Distance (Vector2f, Vector2f)

    test("Vector2DistanceTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(3.0f, 4.0f);

        auto expected = (float)std::sqrt(8);
        float actual;

        actual = FVector2::distance(a, b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Distance did not return the expected value.");
    });

    // A test for Distance (Vector2f, Vector2f)
    // Distance from the same point

    test("Vector2DistanceTest2", [] ()
    {
        auto a = FVector2(1.051f, 2.05f);
        auto b = FVector2(1.051f, 2.05f);

        auto actual = FVector2::distance(a, b);
        Assert::AreEqual(0.0f, actual);
    });

    // A test for DistanceSquared (Vector2f, Vector2f)

    test("Vector2DistanceSquaredTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(3.0f, 4.0f);

        auto expected = 8.0f;
        float actual;

        actual = FVector2::distanceSquared(a, b);
        Assert::True(MathHelper::Equal(expected, actual),
            "Vector2f.DistanceSquared did not return the expected value.");
    });

    // A test for Dot (Vector2f, Vector2f)

    test("Vector2DotTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(3.0f, 4.0f);

        auto expected = 11.0f;
        float actual;

        actual = FVector2::dot(a, b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Dot did not return the expected value.");
    });

    // A test for Dot (Vector2f, Vector2f)
    // Dot test for perpendicular vector

    test("Vector2DotTest1", [] ()
    {
        auto a = FVector2(1.55f, 1.55f);
        auto b = FVector2(-1.55f, 1.55f);

        auto expected = 0.0f;
        auto actual = FVector2::dot(a, b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Dot (Vector2f, Vector2f)
    // Dot test with specail float values

    test("Vector2DotTest2", [] ()
    {
        float min_val = -std::numeric_limits<float>::infinity();
        float max_val = std::numeric_limits<float>::infinity();

        auto a = FVector2(min_val, min_val);
        auto b = FVector2(max_val, max_val);

        auto actual = FVector2::dot(a, b);
        Assert::True(std::isinf(actual) && actual < 0, "Vector2f.Dot did not return the expected value.");
    });

    // A test for Length ()

    test("Vector2LengthTest", [] ()
    {
        auto a = FVector2(2.0f, 4.0f);

        auto target = a;

        auto expected = (float)std::sqrt(20);
        float actual;

        actual = target.length();

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Length did not return the expected value.");
    });

    // A test for Length ()
    // Length test where length is zero

    test("Vector2LengthTest1", [] ()
    {
        auto target = FVector2(0.0f, 0.0f);

        auto expected = 0.0f;
        float actual;

        actual = target.length();

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Length did not return the expected value.");
    });

    // A test for LengthSquared ()

    test("Vector2LengthSquaredTest", [] ()
    {
        auto a = FVector2(2.0f, 4.0f);

        auto target = a;

        auto expected = 20.0f;
        float actual;

        actual = target.lengthSquared();

        Assert::True(MathHelper::Equal(expected, actual),
            "Vector2f.LengthSquared did not return the expected value.");
    });

    // A test for LengthSquared ()
    // LengthSquared test where the result is zero

    test("Vector2LengthSquaredTest1", [] ()
    {
        auto a = FVector2(0.0f, 0.0f);

        auto expected = 0.0f;
        auto actual = a.lengthSquared();

        Assert::AreEqual(expected, actual);
    });

    // A test for Min (Vector2f, Vector2f)

    test("Vector2MinTest", [] ()
    {
        auto a = FVector2(-1.0f, 4.0f);
        auto b = FVector2(2.0f, 1.0f);

        auto expected = FVector2(-1.0f, 1.0f);
        FVector2 actual;
        actual = a.minimum(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Min did not return the expected value.");
    });


    test("Vector2MinMaxCodeCoverageTest", []()
    {
        auto min = FVector2(0, 0);
        auto max = FVector2(1, 1);
        FVector2 actual;

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

    // A test for Max (Vector2f, Vector2f)

    test("Vector2MaxTest", []()
    {
        auto a = FVector2(-1.0f, 4.0f);
        auto b = FVector2(2.0f, 1.0f);

        auto expected = FVector2(2.0f, 4.0f);
        FVector2 actual;
        actual = a.maximum(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Max did not return the expected value.");
    });

    // A test for Clamp (Vector2f, Vector2f, Vector2f)

    test("Vector2ClampTest", [] ()
    {
        auto a = FVector2(0.5f, 0.3f);
        auto min = FVector2(0.0f, 0.1f);
        auto max = FVector2(1.0f, 1.1f);

        // Normal case.
        // Case N1: specified value is in the range.
        auto expected = FVector2(0.5f, 0.3f);
        auto actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");
        // Normal case.
        // Case N2: specified value is bigger than max value.
        a = FVector2(2.0f, 3.0f);
        expected = max;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");
        // Case N3: specified value is smaller than max value.
        a = FVector2(-1.0f, -2.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");
        // Case N4: combination case.
        a = FVector2(-2.0f, 4.0f);
        expected = FVector2(min.X, max.Y);
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");
        // User specified min value is bigger than max value.
        max = FVector2(0.0f, 0.1f);
        min = FVector2(1.0f, 1.1f);

        // Case W1: specified value is in the range.
        a = FVector2(0.5f, 0.3f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");

        // Normal case.
        // Case W2: specified value is bigger than max and min value.
        a = FVector2(2.0f, 3.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");

        // Case W3: specified value is smaller than min and max value.
        a = FVector2(-1.0f, -2.0f);
        expected = min;
        actual = a.clamp(min, max);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Clamp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)

    test("Vector2LerpTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(3.0f, 4.0f);

        auto t = 0.5f;

        auto expected = FVector2(2.0f, 3.0f);
        FVector2 actual;
        actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test with factor zero

    test("Vector2LerpTest1", [] ()
    {
        auto a = FVector2(0.0f, 0.0f);
        auto b = FVector2(3.18f, 4.25f);

        auto t = 0.0f;
        auto expected = FVector2::zero();
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test with factor one

    test("Vector2LerpTest2", [] ()
    {
        auto a = FVector2(0.0f, 0.0f);
        auto b = FVector2(3.18f, 4.25f);

        auto t = 1.0f;
        auto expected = FVector2(3.18f, 4.25f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test with factor > 1

    test("Vector2LerpTest3", [] ()
    {
        auto a = FVector2(0.0f, 0.0f);
        auto b = FVector2(3.18f, 4.25f);

        auto t = 2.0f;
        auto expected = b * 2.0f;
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test with factor < 0

    test("Vector2LerpTest4", []()
    {
        auto a = FVector2(0.0f, 0.0f);
        auto b = FVector2(3.18f, 4.25f);

        auto t = -2.0f;
        auto expected = -(b * 2.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test with special float value

    test("Vector2LerpTest5", [] ()
    {
        auto inf = std::numeric_limits<float>::infinity();
        auto neg_inf = -std::numeric_limits<float>::infinity();
        auto a = FVector2(45.67f, 90.0f);
        auto b = FVector2(inf, neg_inf);

        auto t = 0.408f;
        auto actual = a.lerp(b, t);
        Assert::True(std::isinf(actual.X) && actual.X > 0, "Vector2f.Lerp did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y < 0, "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Lerp (Vector2f, Vector2f, float)
    // Lerp test from the same point

    test("Vector2LerpTest6", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(1.0f, 2.0f);

        auto t = 0.5f;

        auto expected = FVector2(1.0f, 2.0f);
        auto actual = a.lerp(b, t);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Lerp did not return the expected value.");
    });

    // A test for Transform(Vector2f, FMatrix4x4)

    test("Vector2TransformTest", [] ()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector2(10.316987f, 22.183012f);
        FVector2 actual;

        actual = FMatrix4x4::transform(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Transform did not return the expected value.");
    });

    // A test for TransformNormal (Vector2f, FMatrix4x4)

    test("Vector2TransformNormalTest", [] ()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
        m.M41 = 10.0f;
        m.M42 = 20.0f;
        m.M43 = 30.0f;

        auto expected = FVector2(0.3169873f, 2.18301272f);
        FVector2 actual;

        actual = FMatrix4x4::transformNormal(v, m);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Tranform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FQuaternion)

    test("Vector2TransformByQuaternionTest", [] ()
    {
        auto v = FVector2(1.0f, 2.0f);

        auto m =
            FMatrix4x4::rotationX(mathOps::toRadians(30)) *
            FMatrix4x4::rotationY(mathOps::toRadians(30)) *
            FMatrix4x4::rotationZ(mathOps::toRadians(30));
        auto q = m.quaternion();

        auto expected = FMatrix4x4::transform(v, m);
        auto actual = FQuaternion::transform(v, q);
        Assert::True(expected.almostEquals(actual));
    });

    // A test for Transform (Vector2f, FQuaternion)
    // Transform Vector2f with zero quaternion

    test("Vector2TransformByQuaternionTest1", [] ()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto q = FQuaternion();
        auto expected = v;

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Transform did not return the expected value.");
    });

    // A test for Transform (Vector2f, FQuaternion)
    // Transform Vector2f with identity quaternion

    test("Vector2TransformByQuaternionTest2", [] ()
    {
        auto v = FVector2(1.0f, 2.0f);
        auto q = FQuaternion::identity();
        auto expected = v;

        auto actual = FQuaternion::transform(v, q);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Transform did not return the expected value.");
    });

    // A test for Normalize (Vector2f)

    test("Vector2NormalizeTest", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);
        auto expected = FVector2(0.554700196225229122018341733457f, 0.8320502943378436830275126001855f);
        Assert::AreEqual(a.normalize(), expected);
    });

    // A test for Normalize (Vector2f)
    // Normalize zero length vector

    test("Vector2NormalizeTest1", [] ()
    {
        auto a = FVector2(); // no parameter, default to 0.0f
        auto actual = a.normalize();
        Assert::True(std::isnan(actual.X) && std::isnan(actual.Y), "Vector2f.Normalize did not return the expected value.");
    });

    // A test for Normalize (Vector2f)
    // Normalize infinite length vector

    test("Vector2NormalizeTest2", [] ()
    {
        auto max = std::numeric_limits<float>::max();
        auto a = FVector2(max, max);
        auto actual = a.normalize();
        auto expected = FVector2(0, 0);
        Assert::True(expected == actual);
    });

    // A test for operator - (Vector2f)

    test("Vector2UnaryNegationTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);

        auto expected = FVector2(-1.0f, -2.0f);
        FVector2 actual;

        actual = -a;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator - did not return the expected value.");
    });



    // A test for operator - (Vector2f)
    // Negate test with special float value

    test("Vector2UnaryNegationTest1", [] ()
    {
        auto inf = std::numeric_limits<float>::infinity();
        auto neg_inf = -std::numeric_limits<float>::infinity();
        auto a = FVector2(inf, neg_inf);

        auto actual = -a;

        Assert::True(std::isinf(actual.X) && actual.X < 0, "Vector2f.operator - did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y > 0, "Vector2f.operator - did not return the expected value.");
    });

    // A test for operator - (Vector2f)
    // Negate test with special float value

    test("Vector2UnaryNegationTest2", [] ()
    {
        const auto nan = std::numeric_limits<float>::quiet_NaN();
        auto a = FVector2(nan, 0.0f);
        auto actual = -a;

        Assert::True(std::isnan(actual.X), "Vector2f.operator - did not return the expected value.");
        Assert::True(0.0f == actual.Y, "Vector2f.operator - did not return the expected value.");
    });

    // A test for operator - (Vector2f, Vector2f)

    test("Vector2SubtractionTest", [] ()
    {
        auto a = FVector2(1.0f, 3.0f);
        auto b = FVector2(2.0f, 1.5f);

        auto expected = FVector2(-1.0f, 1.5f);
        FVector2 actual;

        actual = a - b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator - did not return the expected value.");
    });

    // A test for operator * (Vector2f, float)

    test("Vector2MultiplyOperatorTest", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);
        const float factor = 2.0f;

        auto expected = FVector2(4.0f, 6.0f);
        FVector2 actual;

        actual = a * factor;
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator * did not return the expected value.");
    });

    // A test for operator * (float, Vector2f)

    test("Vector2MultiplyOperatorTest2", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);
        const float factor = 2.0f;

        auto expected = FVector2(4.0f, 6.0f);
        FVector2 actual;

        actual = factor * a;
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator * did not return the expected value.");
    });

    // A test for operator * (Vector2f, Vector2f)

    test("Vector2MultiplyOperatorTest3", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);
        auto b = FVector2(4.0f, 5.0f);

        auto expected = FVector2(8.0f, 15.0f);
        FVector2 actual;

        actual = a * b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator * did not return the expected value.");
    });

    // A test for operator / (Vector2f, float)

    test("Vector2DivisionTest", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);

        auto div = 2.0f;

        auto expected = FVector2(1.0f, 1.5f);
        FVector2 actual;

        actual = a / div;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector2f, Vector2f)

    test("Vector2DivisionTest1", [] ()
    {
        auto a = FVector2(2.0f, 3.0f);
        auto b = FVector2(4.0f, 5.0f);

        auto expected = FVector2(2.0f / 4.0f, 3.0f / 5.0f);
        FVector2 actual;

        actual = a / b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector2f, float)
    // Divide by zero

    test("Vector2DivisionTest2", [] ()
    {
        auto a = FVector2(-2.0f, 3.0f);

        auto div = 0.0f;

        auto actual = a / div;

        Assert::True(std::isinf(actual.X) && actual.X < 0, "Vector2f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y) && actual.Y > 0, "Vector2f.operator / did not return the expected value.");
    });

    // A test for operator / (Vector2f, Vector2f)
    // Divide by zero

    test("Vector2DivisionTest3", [] ()
    {
        auto a = FVector2(0.047f, -3.0f);
        auto b = FVector2();

        auto actual = a / b;

        Assert::True(std::isinf(actual.X), "Vector2f.operator / did not return the expected value.");
        Assert::True(std::isinf(actual.Y), "Vector2f.operator / did not return the expected value.");
    });

    // A test for operator + (Vector2f, Vector2f)

    test("Vector2AdditionTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(3.0f, 4.0f);

        auto expected = FVector2(4.0f, 6.0f);
        FVector2 actual;

        actual = a + b;

        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.operator + did not return the expected value.");
    });

    // A test for Vector2f (float, float)

    test("Vector2ConstructorTest", [] ()
    {
        auto x = 1.0f;
        auto y = 2.0f;

        auto target = FVector2(x, y);
        Assert::True(MathHelper::Equal(target.X, x) && MathHelper::Equal(target.Y, y), "Vector2f(x,y) constructor did not return the expected value.");
    });

    // A test for Vector2f ()
    // Constructor with no parameter

    test("Vector2ConstructorTest2", [] ()
    {
        auto target = FVector2();
        Assert::AreEqual(target.X, 0.0f);
        Assert::AreEqual(target.Y, 0.0f);
    });

    // A test for Vector2f (float, float)
    // Constructor with special floating values

    test("Vector2ConstructorTest3", [] ()
    {
        const auto nan = std::numeric_limits<float>::quiet_NaN();
        auto max = std::numeric_limits<float>::max();
        auto target = FVector2(nan, max);
        Assert::True(std::isnan(target.X));
        Assert::True(target.Y == max);
    });

    // A test for Vector2f (float)

    test("Vector2ConstructorTest4", [] ()
    {
        auto value = 1.0f;
        auto target = FVector2(value);

        auto expected = FVector2(value, value);
        Assert::AreEqual(expected, target);

        value = 2.0f;
        target = FVector2(value);
        expected = FVector2(value, value);
        Assert::AreEqual(expected, target);
    });

    // A test for Add (Vector2f, Vector2f)

    test("Vector2AddTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(5.0f, 6.0f);

        auto expected = FVector2(6.0f, 8.0f);
        FVector2 actual;

        actual = a.add(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Divide (Vector2f, float)

    test("Vector2DivideTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto div = 2.0f;
        auto expected = FVector2(0.5f, 1.0f);
        Assert::AreEqual(expected, a / div);
    });

    // A test for Divide (Vector2f, Vector2f)

    test("Vector2DivideTest1", [] ()
    {
        auto a = FVector2(1.0f, 6.0f);
        auto b = FVector2(5.0f, 2.0f);

        auto expected = FVector2(1.0f / 5.0f, 6.0f / 2.0f);
        FVector2 actual;

        actual = mathOps::divide(a, b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Equals (object)

    test("Vector2EqualsTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(1.0f, 2.0f);

        // case 1: compare between same values
        auto obj = b;

        auto expected = true;
        bool actual = a == obj;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10.0f);
        obj = b;
        expected = false;
        actual = a == obj;
        Assert::AreEqual(expected, actual);

        //// case 3: compare between different types.
        //auto obj = FQuaternion();
        //expected = false;
        //actual = a == obj;
        //Assert::AreEqual(expected, actual);

        //// case 3: compare against null.
        ////obj = null;
        //expected = false;
        //actual = a == obj;
        Assert::AreEqual(expected, actual);
    });

    // A test for Multiply (Vector2f, float)

    test("Vector2MultiplyTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        const float factor = 2.0f;
        auto expected = FVector2(2.0f, 4.0f);
        Assert::AreEqual(expected, a * factor);
    });

    // A test for Multiply (float, Vector2f)

    test("Vector2MultiplyTest2", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        const float factor = 2.0f;
        auto expected = FVector2(2.0f, 4.0f);
        Assert::AreEqual(expected, factor * a);
    });

    // A test for Multiply (Vector2f, Vector2f)

    test("Vector2MultiplyTest3", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(5.0f, 6.0f);

        auto expected = FVector2(5.0f, 12.0f);
        FVector2 actual;

        actual = a.multiply(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Negate (Vector2f)

    test("Vector2NegateTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);

        auto expected = FVector2(-1.0f, -2.0f);
        FVector2 actual;

        actual = a.negate();
        Assert::AreEqual(expected, actual);
    });

    // A test for operator != (Vector2f, Vector2f)

    test("Vector2InequalityTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(1.0f, 2.0f);

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

    // A test for operator == (Vector2f, Vector2f)

    test("Vector2EqualityTest", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(1.0f, 2.0f);

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

    // A test for Subtract (Vector2f, Vector2f)

    test("Vector2SubtractTest", [] ()
    {
        auto a = FVector2(1.0f, 6.0f);
        auto b = FVector2(5.0f, 2.0f);

        auto expected = FVector2(-4.0f, 4.0f);
        FVector2 actual;

        actual = a.subtract(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for UnitX

    test("Vector2UnitXTest", [] ()
    {
        auto val = FVector2(1.0f, 0.0f);
        Assert::AreEqual(val, FVector2::unitX());
    });

    // A test for UnitY

    test("Vector2UnitYTest", [] ()
    {
        auto val = FVector2(0.0f, 1.0f);
        Assert::AreEqual(val, FVector2::unitY());
    });

    // A test for One

    test("Vector2OneTest", [] ()
    {
        auto val = FVector2(1.0f, 1.0f);
        Assert::AreEqual(val, FVector2::one());
    });

    // A test for Zero

    test("Vector2ZeroTest", [] ()
    {
        auto val = FVector2(0.0f, 0.0f);
        Assert::AreEqual(val, FVector2::zero());
    });

    // A test for Equals (Vector2f)

    test("Vector2EqualsTest1", [] ()
    {
        auto a = FVector2(1.0f, 2.0f);
        auto b = FVector2(1.0f, 2.0f);

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == (b);
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10);
        expected = false;
        actual = a == (b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Vector2f comparison involving NaN values

    test("Vector2EqualsNanTest", [] ()
    {
        const auto nan = std::numeric_limits<float>::quiet_NaN();
        auto a = FVector2(nan, 0);
        auto b = FVector2(0, nan);

        Assert::False(a == FVector2::zero());
        Assert::False(b == FVector2::zero());

        Assert::True(a != FVector2::zero());
        Assert::True(b != FVector2::zero());

        Assert::False(a == (FVector2::zero()));
        Assert::False(b == (FVector2::zero()));

        // Counterintuitive result - IEEE rules for NaN comparison are weird!
        Assert::False(a == (a));
        Assert::False(b == (b));
    });

    // A test for Reflect (Vector2f, Vector2f)

    test("Vector2ReflectTest", [] ()
    {
        auto a = FVector2(1.0f, 1.0f).normalize();

        // Reflect on XZ plane.
        auto n = FVector2(0.0f, 1.0f);
        auto expected = FVector2(a.X, -a.Y);
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Reflect did not return the expected value.");

        // Reflect on XY plane.
        n = FVector2(0.0f, 0.0f);
        expected = FVector2(a.X, a.Y);
        actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Reflect did not return the expected value.");

        // Reflect on YZ plane.
        n = FVector2(1.0f, 0.0f);
        expected = FVector2(-a.X, a.Y);
        actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Reflect did not return the expected value.");
    });

    // A test for Reflect (Vector2f, Vector2f)
    // Reflection when normal and source are the same

    test("Vector2ReflectTest1", [] ()
    {
        auto n = FVector2(0.45f, 1.28f);
        n = n.normalize();
        auto a = n;

        auto expected = -n;
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Reflect did not return the expected value.");
    });

    // A test for Reflect (Vector2f, Vector2f)
    // Reflection when normal and source are negation

    test("Vector2ReflectTest2", [] ()
    {
        auto n = FVector2(0.45f, 1.28f);
        n = n.normalize();
        auto a = -n;

        auto expected = n;
        auto actual = a.reflect(n);
        Assert::True(MathHelper::Equal(expected, actual), "Vector2f.Reflect did not return the expected value.");
    });


    test("Vector2AbsTest", [] () {
        auto neg_inf = -std::numeric_limits<float>::infinity();
        auto v1 = FVector2(-2.5f, 2.0f);
        auto v3 = FVector2(0.0f, neg_inf).abs();
        auto v = v1.abs();
        Assert::True(2.5f == v.X);
        Assert::True(2.0f == v.Y);
        Assert::True(0.0f == v3.X);
        Assert::True(std::isinf(v3.Y) && v3.Y > 0);
    });


    test("Vector2SqrtTest", [] ()
    {
        auto nan_val = std::numeric_limits<float>::quiet_NaN();
        auto v1 = FVector2(-2.5f, 2.0f);
        auto v2 = FVector2(5.5f, 4.5f);
        Assert::True(2 == (int)v2.squareRoot().X);
        Assert::True(2 == (int)v2.squareRoot().Y);
        Assert::True(std::isnan(v1.squareRoot().X));
    });

#pragma endregion

    return 0;
}