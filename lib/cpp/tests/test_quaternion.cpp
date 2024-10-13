#include "test_vim_math3d.h"

int main()
{
#pragma region QuaternionTests
    std::cout << "FQuaternion Tests" << std::endl;
    // A test for Dot (FQuaternion, FQuaternion)

    test("QuaternionDotTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = 70.0f;

        auto actual = FQuaternion::dot(a, b);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::Dot did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Length ()

    test("QuaternionLengthTest", []()  {
        auto v = FVector3(1.0f, 2.0f, 3.0f);

        auto w = 4.0f;

        auto target = FQuaternion(v, w);

        auto expected = 5.477226f;

        auto actual = target.length();

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::Length did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for LengthSquared ()

    test("QuaternionLengthSquaredTest", []()
            {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto w = 4.0f;

        auto target = FQuaternion(v, w);

        auto expected = 30.0f;

        auto actual = target.lengthSquared();

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::LengthSquared did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Lerp (FQuaternion, FQuaternion, float)

    test("QuaternionLerpTest", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 0.5f;

        auto expected = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(20.0f));

        auto actual = FQuaternion::lerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::lerp did not return the expected value: expected {expected} actual {actual}");

        // Case a and b are same.
        expected = a;
        actual = FQuaternion::lerp(a, a, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::lerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Lerp (FQuaternion, FQuaternion, float)
    // Lerp test when t = 0

    test("QuaternionLerpTest1", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 0.0f;

        auto expected = FQuaternion(a.X, a.Y, a.Z, a.W);
        auto actual = FQuaternion::lerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::lerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Lerp (FQuaternion, FQuaternion, float)
    // Lerp test when t = 1

    test("QuaternionLerpTest2", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 1.0f;

        auto expected = FQuaternion(b.X, b.Y, b.Z, b.W);
        auto actual = FQuaternion::lerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::lerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Lerp (FQuaternion, FQuaternion, float)
    // Lerp test when the two quaternions are more than 90 degree (dot product <0)

    test("QuaternionLerpTest3", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = -a;

        auto t = 1.0f;

        auto actual = FQuaternion::lerp(a, b, t);
        // Note that in quaternion world, Q == -Q. In the case of quaternions dot product is zero, 
        // one of the quaternion will be flipped to compute the shortest distance. When t = 1, we
        // expect the result to be the same as quaternion b but flipped.
        Assert::True(actual == a, "FQuaternion::lerp did not return the expected value: expected {a} actual {actual}");
    });

    // A test for Conjugate(FQuaternion)

    test("QuaternionConjugateTest1", []()
            {
        auto a = FQuaternion(1, 2, 3, 4);

        auto expected = FQuaternion(-1, -2, -3, 4);

        auto actual = a.conjugate();
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::Conjugate did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Normalize (FQuaternion)

    test("QuaternionNormalizeTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);

        auto expected = FQuaternion(0.182574168f, 0.365148336f, 0.5477225f, 0.7302967f);

        auto actual = a.normalize();
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::Normalize did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Normalize (FQuaternion)
    // Normalize zero length quaternion

    test("QuaternionNormalizeTest1", []()  {
        auto a = FQuaternion(0.0f, 0.0f, -0.0f, 0.0f);

        auto actual = a.normalize();
        Assert::True(std::isnan(actual.X) && std::isnan(actual.Y) && std::isnan(actual.Z) && std::isnan(actual.W)
            , "FQuaternion::Normalize did not return the expected value: expected {FQuaternion(std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN())} actual {actual}");
    });

    // A test for Concatenate(FQuaternion, FQuaternion)

    test("QuaternionConcatenateTest1", []()
            {
        auto b = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto a = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FQuaternion(24.0f, 48.0f, 48.0f, -6.0f);

        auto actual = FQuaternion::concatenate(a, b);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::Concatenate did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for operator - (FQuaternion, FQuaternion)

    test("QuaternionSubtractionTest", []()
            {
        auto a = FQuaternion(1.0f, 6.0f, 7.0f, 4.0f);
        auto b = FQuaternion(5.0f, 2.0f, 3.0f, 8.0f);

        auto expected = FQuaternion(-4.0f, 4.0f, 4.0f, -4.0f);

        auto actual = a - b;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator - did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for operator * (FQuaternion, float)

    test("QuaternionMultiplyTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto factor = 0.5f;

        auto expected = FQuaternion(0.5f, 1.0f, 1.5f, 2.0f);

        auto actual = a * factor;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator * did not return the expected value: expected {expected} actual {actual}");
    });

            // A test for operator * (FQuaternion, FQuaternion)
        
    test("QuaternionMultiplyTest1", []()
    {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FQuaternion(24.0f, 48.0f, 48.0f, -6.0f);

        auto actual = a * b;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator * did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for operator / (FQuaternion, FQuaternion)

    test("QuaternionDivisionTest1", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FQuaternion(-0.045977015f, -0.09195402f, -7.450581E-9f, 0.402298868f);

        auto actual = a / b;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator / did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for operator + (FQuaternion, FQuaternion)

    test("QuaternionAdditionTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FQuaternion(6.0f, 8.0f, 10.0f, 12.0f);

        auto actual = a + b;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator + did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for FQuaternion (float, float, float, float)

    test("QuaternionConstructorTest", []()
            {
        auto x = 1.0f;
        auto y = 2.0f;
        auto z = 3.0f;
        auto w = 4.0f;

        auto target = FQuaternion(x, y, z, w);

        Assert::True(MathHelper::Equal(target.X, x) && MathHelper::Equal(target.Y, y) && MathHelper::Equal(target.Z, z) && MathHelper::Equal(target.W, w),
            "FQuaternion::constructor (x,y,z,w) did not return the expected value.");
    });

    // A test for FQuaternion (Vector3f, float)

    test("QuaternionConstructorTest1", []()
            {
        auto v = FVector3(1.0f, 2.0f, 3.0f);
        auto w = 4.0f;

        auto target = FQuaternion(v, w);
        Assert::True(MathHelper::Equal(target.X, v.X) && MathHelper::Equal(target.Y, v.Y) && MathHelper::Equal(target.Z, v.Z) && MathHelper::Equal(target.W, w),
            "FQuaternion::constructor (Vector3f,w) did not return the expected value.");
    });

    // A test for CreateFromAxisAngle (Vector3f, float)

    test("QuaternionCreateFromAxisAngleTest", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto angle = MathHelper::ToRadians(30.0f);

        auto expected = FQuaternion(0.0691723f, 0.1383446f, 0.207516879f, 0.9659258f);

        auto actual = FQuaternion::fromAxisAngle(axis, angle);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::fromAxisAngle did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for CreateFromAxisAngle (Vector3f, float)
    // CreateFromAxisAngle of zero vector

    test("QuaternionCreateFromAxisAngleTest1", []() {
        auto axis = FVector3();
        auto angle = MathHelper::ToRadians(-30.0f);

        auto cos = (float)std::cos(angle / 2.0f);
        auto actual = FQuaternion::fromAxisAngle(axis, angle);

        Assert::True(actual.X == 0.0f && actual.Y == 0.0f && actual.Z == 0.0f && MathHelper::Equal(cos, actual.W)
            , "FQuaternion::fromAxisAngle did not return the expected value.");
    });

    // A test for CreateFromAxisAngle (Vector3f, float)
    // CreateFromAxisAngle of angle = 30 && 750

    test("QuaternionCreateFromAxisAngleTest2", []() {
        auto axis = FVector3(1, 0, 0);
        auto angle1 = MathHelper::ToRadians(30.0f);
        auto angle2 = MathHelper::ToRadians(750.0f);

        auto actual1 = FQuaternion::fromAxisAngle(axis, angle1);
        auto actual2 = FQuaternion::fromAxisAngle(axis, angle2);
        Assert::True(MathHelper::Equal(actual1, actual2), "FQuaternion::fromAxisAngle did not return the expected value: actual1 {actual1} actual2 {actual2}");
    });

    // A test for CreateFromAxisAngle (Vector3f, float)
    // CreateFromAxisAngle of angle = 30 && 390

    test("QuaternionCreateFromAxisAngleTest3", []() {
        auto axis = FVector3(1, 0, 0);
        auto angle1 = MathHelper::ToRadians(30.0f);
        auto angle2 = MathHelper::ToRadians(390.0f);

        auto actual1 = FQuaternion::fromAxisAngle(axis, angle1);
        auto actual2 = FQuaternion::fromAxisAngle(axis, angle2);
        actual1 = actual1.setX(-actual1.X);
        actual1 = actual1.setW(-actual1.W);

        Assert::True(MathHelper::Equal(actual1, actual2), "FQuaternion::fromAxisAngle did not return the expected value: actual1 {actual1} actual2 {actual2}");
    });


    test("QuaternionCreateFromYawPitchRollTest1", []()
            {
        auto yawAngle = MathHelper::ToRadians(30.0f);
        auto pitchAngle = MathHelper::ToRadians(40.0f);
        auto rollAngle = MathHelper::ToRadians(50.0f);

        auto yaw = FQuaternion::fromAxisAngle(FVector3::unitY(), yawAngle);
        auto pitch = FQuaternion::fromAxisAngle(FVector3::unitX(), pitchAngle);
        auto roll = FQuaternion::fromAxisAngle(FVector3::unitZ(), rollAngle);

        auto expected = yaw * pitch * roll;
        auto actual = FQuaternion::fromYawPitchRoll(yawAngle, pitchAngle, rollAngle);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::QuaternionCreateFromYawPitchRollTest1 did not return the expected value: expected {expected} actual {actual}");
    });

    // Covers more numeric rigions

    test("QuaternionCreateFromYawPitchRollTest2", []()
            {
        const float step = 35.0f;

        for (auto yawAngle = -720.0f; yawAngle <= 720.0f; yawAngle += step)
        {
            for (auto pitchAngle = -720.0f; pitchAngle <= 720.0f; pitchAngle += step)
            {
                for (auto rollAngle = -720.0f; rollAngle <= 720.0f; rollAngle += step)
                {
                    auto yawRad = MathHelper::ToRadians(yawAngle);
                    auto pitchRad = MathHelper::ToRadians(pitchAngle);
                    auto rollRad = MathHelper::ToRadians(rollAngle);

                    auto yaw = FQuaternion::fromAxisAngle(FVector3::unitY(), yawRad);
                    auto pitch = FQuaternion::fromAxisAngle(FVector3::unitX(), pitchRad);
                    auto roll = FQuaternion::fromAxisAngle(FVector3::unitZ(), rollRad);

                    auto expected = yaw * pitch * roll;
                    auto actual = FQuaternion::fromYawPitchRoll(yawRad, pitchRad, rollRad);
                    Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::QuaternionCreateFromYawPitchRollTest2 Yaw:{yawAngle} Pitch:{pitchAngle} Roll:{rollAngle} did not return the expected value: expected {expected} actual {actual}");
                }
            }
        }
    });

    // A test for Slerp (FQuaternion, FQuaternion, float)

    test("QuaternionSlerpTest", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 0.5f;

        auto expected = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(20.0f));

        auto actual = FQuaternion::slerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");

        // Case a and b are same.
        expected = a;
        actual = FQuaternion::slerp(a, a, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Slerp (FQuaternion, FQuaternion, float)
    // Slerp test where t = 0

    test("QuaternionSlerpTest1", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 0.0f;

        auto expected = FQuaternion(a.X, a.Y, a.Z, a.W);
        auto actual = FQuaternion::slerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Slerp (FQuaternion, FQuaternion, float)
    // Slerp test where t = 1

    test("QuaternionSlerpTest2", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 1.0f;

        auto expected = FQuaternion(b.X, b.Y, b.Z, b.W);
        auto actual = FQuaternion::slerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Slerp (FQuaternion, FQuaternion, float)
    // Slerp test where dot product is < 0

    test("QuaternionSlerpTest3", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = -a;

        auto t = 1.0f;

        auto expected = a;
        auto actual = FQuaternion::slerp(a, b, t);
        // Note that in quaternion world, Q == -Q. In the case of quaternions dot product is zero, 
        // one of the quaternion will be flipped to compute the shortest distance. When t = 1, we
        // expect the result to be the same as quaternion b but flipped.
        Assert::True(actual == expected, "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Slerp (FQuaternion, FQuaternion, float)
    // Slerp test where the quaternion is flipped

    test("QuaternionSlerpTest4", []()
            {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto a = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(10.0f));
        auto b = -FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto t = 0.0f;

        auto expected = FQuaternion(a.X, a.Y, a.Z, a.W);
        auto actual = FQuaternion::slerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::slerp did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for operator - (FQuaternion)

    test("QuaternionUnaryNegationTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);

        auto expected = FQuaternion(-1.0f, -2.0f, -3.0f, -4.0f);

        auto actual = -a;

        Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::operator - did not return the expected value: expected {expected} actual {actual}");
    });

    // A test for Inverse (FQuaternion)

    test("QuaternionInverseTest", []()
            {
        auto a = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);

        auto expected = FQuaternion(-0.0287356321f, -0.03448276f, -0.0402298868f, 0.04597701f);

        auto actual = a.inverse();
        Assert::True(expected == actual);
    });

    // A test for Inverse (FQuaternion)
    // Invert zero length quaternion

    test("QuaternionInverseTest1", []()
            {
        auto a = FQuaternion();
        auto actual = a.inverse();

        Assert::True(std::isnan(actual.X) && std::isnan(actual.Y) && std::isnan(actual.Z) && std::isnan(actual.W)
            , "FQuaternion::Inverse - did not return the expected value: expected {FQuaternion(std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN(), std::numeric_limits<float>::quiet_NaN())} actual {actual}");
    });

    // A test for Add (FQuaternion, FQuaternion)

    test("QuaternionAddTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);
        auto expected = FQuaternion(6.0f, 8.0f, 10.0f, 12.0f);
        Assert::True(expected == (a + b));
    });

    // A test for Divide (FQuaternion, FQuaternion)

    test("QuaternionDivideTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);
        auto expected = FQuaternion(-0.045977015f, -0.09195402f, -7.450581E-9f, 0.402298868f);
        Assert::IsTrue(MathHelper::Equal(expected, a / b));
    });

    // A test for Multiply (FQuaternion, float)

    test("QuaternionMultiplyTest2", []() {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto factor = 0.5f;
        auto expected = FQuaternion(0.5f, 1.0f, 1.5f, 2.0f);
        Assert::True(expected == (a * factor));
    });

    // A test for Multiply (FQuaternion, FQuaternion)

    test("QuaternionMultiplyTest3", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(5.0f, 6.0f, 7.0f, 8.0f);
        auto expected = FQuaternion(24.0f, 48.0f, 48.0f, -6.0f);
        Assert::True(expected == (a * b));
    });

    // A test for Negate (FQuaternion)

    test("QuaternionNegateTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto expected = FQuaternion(-1.0f, -2.0f, -3.0f, -4.0f);
        Assert::True(expected == (- a));
    });

    // A test for Subtract (FQuaternion, FQuaternion)

    test("QuaternionSubtractTest", []()
            {
        auto a = FQuaternion(1.0f, 6.0f, 7.0f, 4.0f);
        auto b = FQuaternion(5.0f, 2.0f, 3.0f, 8.0f);

        auto expected = FQuaternion(-4.0f, 4.0f, 4.0f, -4.0f);
        Assert::True(expected == (a - b));
    });

    // A test for operator != (FQuaternion, FQuaternion)

    test("QuaternionInequalityTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = false;
        auto actual = a != b;
        Assert::True(expected == actual);

        // case 2: compare between different values
        expected = true;
        actual = a != b.setX(10.0f);
        Assert::True(expected == actual);
    });

    // A test for operator == (FQuaternion, FQuaternion)

    test("QuaternionEqualityTest", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10.0f);
        expected = false;
        actual = a == b;
        Assert::True(expected == actual);
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Convert Identity matrix test

    test("QuaternionFromRotationMatrixTest1", []()
            {
        auto matrix = FMatrix4x4::identity();

        auto expected = FQuaternion(0.0f, 0.0f, 0.0f, 1.0f);
        auto actual = matrix.quaternion();
        Assert::True(MathHelper::Equal(expected, actual),
            "FQuaternion::fromRotationMatrix did not return the expected value: expected {expected} actual {actual}");

        // make sure convert back to matrix is same as we passed matrix.
        auto m2 = FMatrix4x4::fromQuaternion(actual);
        Assert::True(MathHelper::Equal(matrix, m2),
            "FQuaternion::fromQuaternion did not return the expected value: matrix {matrix} m2 {m2}");
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Convert X axis rotation matrix

    test("QuaternionFromRotationMatrixTest2", []()
            {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto matrix = FMatrix4x4::rotationX(angle);

            auto expected = FQuaternion::fromAxisAngle(FVector3::unitX(), angle);
            auto actual = matrix.quaternion();
            Assert::True(MathHelper::EqualRotation(expected, actual),
                "FQuaternion::fromRotationMatrix angle:{angle} did not return the expected value: expected {expected} actual {actual}");

            // make sure convert back to matrix is same as we passed matrix.
            auto m2 = FMatrix4x4::fromQuaternion(actual); 
            Assert::True(MathHelper::Equal(matrix, m2),
                "FQuaternion::fromQuaternion angle:{angle} did not return the expected value: matrix {matrix} m2 {m2}");
        }
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Convert Y axis rotation matrix

    test("QuaternionFromRotationMatrixTest3", []()
            {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto matrix = FMatrix4x4::rotationY(angle);

            auto expected = FQuaternion::fromAxisAngle(FVector3::unitY(), angle);
            auto actual = matrix.quaternion();
            Assert::True(MathHelper::EqualRotation(expected, actual),
                "FQuaternion::fromRotationMatrix angle:{angle} did not return the expected value: expected {expected} actual {actual}");

            // make sure convert back to matrix is same as we passed matrix.
            auto m2 = FMatrix4x4::fromQuaternion(actual);
            Assert::True(MathHelper::Equal(matrix, m2),
                "FQuaternion::fromQuaternion angle:{angle} did not return the expected value: matrix {matrix} m2 {m2}");
        }
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Convert Z axis rotation matrix

    test("QuaternionFromRotationMatrixTest4", []()
            {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto matrix = FMatrix4x4::rotationZ(angle);

            auto expected = FQuaternion::fromAxisAngle(FVector3::unitZ(), angle);
            auto actual = matrix.quaternion();
            Assert::True(MathHelper::EqualRotation(expected, actual),
                "FQuaternion::fromRotationMatrix angle:{angle} did not return the expected value: expected {expected} actual {actual}");

            // make sure convert back to matrix is same as we passed matrix.
            auto m2 = FMatrix4x4::fromQuaternion(actual);
            Assert::True(MathHelper::Equal(matrix, m2),
                "FQuaternion::fromQuaternion angle:{angle} did not return the expected value: matrix {matrix} m2 {m2}");
        }
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Convert XYZ axis rotation matrix

    test("QuaternionFromRotationMatrixTest5", []() {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto matrix = FMatrix4x4::rotationX(angle) * FMatrix4x4::rotationY(angle) * FMatrix4x4::rotationZ(angle);

            auto expected =
                FQuaternion::fromAxisAngle(FVector3::unitZ(), angle) *
                FQuaternion::fromAxisAngle(FVector3::unitY(), angle) *
                FQuaternion::fromAxisAngle(FVector3::unitX(), angle);

            auto actual = matrix.quaternion();
            Assert::True(MathHelper::EqualRotation(expected, actual),
                "FQuaternion::fromRotationMatrix angle:{angle} did not return the expected value: expected {expected} actual {actual}");

            // make sure convert back to matrix is same as we passed matrix.
            auto m2 = FMatrix4x4::fromQuaternion(actual);
            Assert::True(MathHelper::Equal(matrix, m2),
                "FQuaternion::fromQuaternion angle:{angle} did not return the expected value: matrix {matrix} m2 {m2}");
        }
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // X axis is most large axis case

    test("QuaternionFromRotationMatrixWithScaledMatrixTest1", []() {
        auto angle = MathHelper::ToRadians(180.0f);
        auto matrix = FMatrix4x4::rotationY(angle) * FMatrix4x4::rotationZ(angle);

        auto expected = FQuaternion::fromAxisAngle(FVector3::unitZ(), angle) * FQuaternion::fromAxisAngle(FVector3::unitY(), angle);
        auto actual = matrix.quaternion();
        Assert::True(MathHelper::EqualRotation(expected, actual),
            "FQuaternion::fromRotationMatrix did not return the expected value: expected {expected} actual {actual}");

        // make sure convert back to matrix is same as we passed matrix.
        auto m2 = FMatrix4x4::fromQuaternion(actual);
        Assert::True(MathHelper::Equal(matrix, m2),
            "FQuaternion::fromQuaternion did not return the expected value: matrix {matrix} m2 {m2}");
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Y axis is most large axis case

    test("QuaternionFromRotationMatrixWithScaledMatrixTest2", []() {
        auto angle = MathHelper::ToRadians(180.0f);
        auto matrix = FMatrix4x4::rotationX(angle) * FMatrix4x4::rotationZ(angle);

        auto expected = FQuaternion::fromAxisAngle(FVector3::unitZ(), angle) * FQuaternion::fromAxisAngle(FVector3::unitX(), angle);
        auto actual = matrix.quaternion();
        Assert::True(MathHelper::EqualRotation(expected, actual),
            "FQuaternion::fromRotationMatrix did not return the expected value: expected {expected} actual {actual}");

        // make sure convert back to matrix is same as we passed matrix.
        auto m2 = FMatrix4x4::fromQuaternion(actual);
        Assert::True(MathHelper::Equal(matrix, m2),
            "FQuaternion::fromQuaternion did not return the expected value: matrix {matrix} m2 {m2}");
    });

    // A test for CreateFromRotationMatrix (FMatrix4x4)
    // Z axis is most large axis case

    test("QuaternionFromRotationMatrixWithScaledMatrixTest3", []() {
        auto angle = MathHelper::ToRadians(180.0f);
        auto matrix = FMatrix4x4::rotationX(angle) * FMatrix4x4::rotationY(angle);

        auto expected = FQuaternion::fromAxisAngle(FVector3::unitY(), angle) * FQuaternion::fromAxisAngle(FVector3::unitX(), angle);
        auto actual = matrix.quaternion();
        Assert::True(MathHelper::EqualRotation(expected, actual),
            "FQuaternion::fromRotationMatrix did not return the expected value: expected {expected} actual {actual}");

        // make sure convert back to matrix is same as we passed matrix.
        auto m2 = FMatrix4x4::fromQuaternion(actual);
        Assert::True(MathHelper::Equal(matrix, m2),
            "FQuaternion::fromQuaternion did not return the expected value: matrix {matrix} m2 {m2}");
    });

    // A test for Equals (FQuaternion)

    test("QuaternionEqualsTest1", []()
            {
        auto a = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);
        auto b = FQuaternion(1.0f, 2.0f, 3.0f, 4.0f);

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b = b.setX(10.0f);
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for Identity

    test("QuaternionIdentityTest", []()
            {
        auto val = FQuaternion(0, 0, 0, 1);
        Assert::True(val == FQuaternion::identity());
    });

    // A test for isIdentity()

    test("QuaternionIsIdentityTest", [](){
        Assert::True(FQuaternion::identity().isIdentity());
        Assert::True(FQuaternion(0, 0, 0, 1).isIdentity());
        Assert::False(FQuaternion(1, 0, 0, 1).isIdentity());
        Assert::False(FQuaternion(0, 1, 0, 1).isIdentity());
        Assert::False(FQuaternion(0, 0, 1, 1).isIdentity());
        Assert::False(FQuaternion(0, 0, 0, 0).isIdentity());
    });

    // A test for FQuaternion comparison involving NaN values

    test("QuaternionEqualsNanTest", [](){
        const auto nan = std::numeric_limits<float>::quiet_NaN();
        auto a = FQuaternion(nan, 0, 0, 0);
        auto b = FQuaternion(0, nan, 0, 0);
        auto c = FQuaternion(0, 0, nan, 0);
        auto d = FQuaternion(0, 0, 0, nan);

        Assert::False(a == FQuaternion(0, 0, 0, 0));
        Assert::False(b == FQuaternion(0, 0, 0, 0));
        Assert::False(c == FQuaternion(0, 0, 0, 0));
        Assert::False(d == FQuaternion(0, 0, 0, 0));

        Assert::True(a != FQuaternion(0, 0, 0, 0));
        Assert::True(b != FQuaternion(0, 0, 0, 0));
        Assert::True(c != FQuaternion(0, 0, 0, 0));
        Assert::True(d != FQuaternion(0, 0, 0, 0));

        Assert::False(a == (FQuaternion(0, 0, 0, 0)));
        Assert::False(b == (FQuaternion(0, 0, 0, 0)));
        Assert::False(c == (FQuaternion(0, 0, 0, 0)));
        Assert::False(d == (FQuaternion(0, 0, 0, 0)));

        Assert::False(a.isIdentity());
        Assert::False(b.isIdentity());
        Assert::False(c.isIdentity());
        Assert::False(d.isIdentity());

        // Counterintuitive result - IEEE rules for NaN comparison are weird!
        Assert::False(a == (a));
        Assert::False(b == (b));
        Assert::False(c == (c));
        Assert::False(d == (d));
    });

    test("ToEulerAndBack", []() {
        auto x = vim::math3d::constants::pi / 5;
        auto y = vim::math3d::constants::pi * 2 / 7;
        auto z = vim::math3d::constants::pi / 3;
        auto euler = FVector3(x, y, z);
        auto quat = FQuaternion::fromEulerAngles(euler);
        auto euler2 = quat.toEulerAngles();
        Assert::True(std::fabs(euler2.Y - euler.Y) < 0.001f);
        Assert::True(std::fabs(euler2.Z - euler.Z) < 0.001f);
    });

    // A test to avoid a floating point NaN precision issue
    // when two input normalized vectors are almost parallel
    // and pointing in the same direction.

    test("CreateRotationFromAtoBTest", []() {
        auto a = FVector3(0.57731324f, 0.57728577f, 0.5774519f);
        auto b = FVector3(0.57738256f, 0.57728577f, 0.57738256f);

        // Assert precondition that a and b are normalized.
        Assert::IsTrue(vim::math3d::mathOps::almostEquals(a.normalize().length(), a.length()));
        Assert::IsTrue(vim::math3d::mathOps::almostEquals(b.normalize().length(), b.length()));

        // Validate that the returned quaternion does not contain NaN due to precision issues
        auto quat = FQuaternion::rotationFromAToB(a, b);
        Assert::IsTrue(quat == (FQuaternion::identity()));
    });
#pragma endregion

    return 0;
}