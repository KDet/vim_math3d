#pragma once

#include <iostream>
#include <sstream>
#include <vector>
#include <array>
#include <numeric>
#include <functional>
#include <initializer_list>
#include <optional>
#include <cassert>
#include <cstdlib>
#include <ctime>
#include <chrono> 

#include "../lib/vim_math3d.h";

using namespace vim::math3d; 

void test(char const* const name, const std::function<void(void)>& func) {
    try { func();  std::cout << "Test " << name << " is passed." << std::endl; assert(true); }
    catch (const std::exception& ex) { std::cout << "!!! Test " << name << " is failed !!!" << ex.what() << std::endl; assert(false); }
};
template<typename T>
void testException(char const* const name, const std::function<void(void)>& func) {
    try { func(); std::cout << "!!! Test " << name << " is failed !!!" << std::endl; assert(false); }
    catch (const T& ex) { std::cout << "Test " << name << " is passed." << std::endl; assert(true); }
};

struct Assert {
    inline static void IsFalse(const bool& value) { assert(!value,  "Assertion is failed"); }
    inline static void IsTrue(const bool& value) { assert(value, "Assertion is failed"); } 
    template<typename T> inline static void AreEqual(const T& should, const T& is) { assert(should == is, "Assertion is failed"); }
    template<typename T> inline static void AreNotEqual(const T& should, const T& is) { assert(should != is, "Assertion is failed"); }
    inline static void True(const bool& value, char const* const message = "Assertion is failed") { assert(value, message); }
    inline static void False(const bool& value, char const* const message = "Assertion is failed") { assert(!value, message); }
};

struct MathHelper
{
    static constexpr float pi = 3.14159265358979323846;
    static constexpr float piOver2 = pi / 2;
    static constexpr float piOver4 = pi / 4;

    template<typename T> static bool Equal(T a, T b) { return (std::fabs(a - b) < 1e-5); }

    static float ToRadians(float degrees) { return degrees * pi / 180.0; }
    
    //static bool Equal(double a, double b) { return (std::fabs(a - b) < 1e-5);}
    template<typename T> static bool Equal(const Vector2<T>& a, const Vector2<T>& b) { return Equal(a.X, b.X) && Equal(a.Y, b.Y); }
    template<typename T> static bool Equal(const Vector3<T>& a, const Vector3<T>& b) { return Equal(a.X, b.X) && Equal(a.Y, b.Y) && Equal(a.Z, b.Z); }
    template<typename T> static bool Equal(const Vector4<T>& a, const Vector4<T>& b) { return Equal(a.X, b.X) && Equal(a.Y, b.Y) && Equal(a.Z, b.Z) && Equal(a.W, b.W); }
    template<typename T> static bool Equal(const Matrix4x4<T>& a, const Matrix4x4<T>& b) {
        return Equal(a.M11, b.M11) && Equal(a.M12, b.M12) && Equal(a.M13, b.M13) && Equal(a.M14, b.M14) &&
            Equal(a.M21, b.M21) && Equal(a.M22, b.M22) && Equal(a.M23, b.M23) && Equal(a.M24, b.M24) &&
            Equal(a.M31, b.M31) && Equal(a.M32, b.M32) && Equal(a.M33, b.M33) && Equal(a.M34, b.M34) &&
            Equal(a.M41, b.M41) && Equal(a.M42, b.M42) && Equal(a.M43, b.M43) && Equal(a.M44, b.M44);
    }
    template<typename T> static bool Equal(const Plane<T>& a, const Plane<T>& b) { return Equal(a.Normal, b.Normal) && Equal(a.D, b.D); }
    template<typename T>  static bool Equal(const Quaternion<T>& a, const Quaternion<T>& b) { return Equal(a.X, b.X) && Equal(a.Y, b.Y) && Equal(a.Z, b.Z) && Equal(a.W, b.W); }
    template<typename T> static bool EqualRotation(const Quaternion<T>& a, const Quaternion<T>& b) { return Equal(a, b) || Equal(a, -b); }
};

FMatrix4x4 GenerateMatrixNumberFrom1To16() {
    auto a = FMatrix4x4();
    a.M11 = 1.0f;
    a.M12 = 2.0f;
    a.M13 = 3.0f;
    a.M14 = 4.0f;
    a.M21 = 5.0f;
    a.M22 = 6.0f;
    a.M23 = 7.0f;
    a.M24 = 8.0f;
    a.M31 = 9.0f;
    a.M32 = 10.0f;
    a.M33 = 11.0f;
    a.M34 = 12.0f;
    a.M41 = 13.0f;
    a.M42 = 14.0f;
    a.M43 = 15.0f;
    a.M44 = 16.0f;
    return a;
};

FMatrix4x4 GenerateTestMatrix() {
    auto m =
        FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
        FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
        FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));
    return m.setTranslation(FVector3(111.0f, 222.0f, 333.0f));
};

void DecomposeTest(float yaw, float pitch, float roll, const FVector3& expectedTranslation, const FVector3& expectedScales)
{
    auto expectedRotation = FQuaternion::fromYawPitchRoll(MathHelper::ToRadians(yaw),
        MathHelper::ToRadians(pitch),
        MathHelper::ToRadians(roll));

    auto m = FMatrix4x4::scale(expectedScales) *
        FMatrix4x4::fromQuaternion(expectedRotation) *
        FMatrix4x4::translation(expectedTranslation);

    FVector3 scales;
    FQuaternion rotation;
    FVector3 translation;

    auto actualResult = m.decompose(scales, rotation, translation);
    Assert::True(actualResult, "FMatrix4x4::Decompose did not return expected value.0");

    auto scaleIsZeroOrNegative = expectedScales.X <= 0 ||
        expectedScales.Y <= 0 ||
        expectedScales.Z <= 0;

    if (scaleIsZeroOrNegative)
    {
        Assert::True(MathHelper::Equal(std::abs(expectedScales.X), std::abs(scales.X)), "FMatrix4x4::Decompose did not return expected value. 1");
        Assert::True(MathHelper::Equal(std::abs(expectedScales.Y), std::abs(scales.Y)), "FMatrix4x4::Decompose did not return expected value. 2");
        Assert::True(MathHelper::Equal(std::abs(expectedScales.Z), std::abs(scales.Z)), "FMatrix4x4::Decompose did not return expected value. 3");
    }
    else
    {
        Assert::True(MathHelper::Equal(expectedScales, scales), "FMatrix4x4::Decompose did not return expected value. 1");
        Assert::True(MathHelper::EqualRotation(expectedRotation, rotation), "FMatrix4x4::Decompose did not return expected value. 2");
    }

    Assert::True(MathHelper::Equal(expectedTranslation, translation), "FMatrix4x4::Decompose did not return expected value.");
};

void ExtractScaleTest(const FVector3& s, const FVector3& r)
{
    auto m = FMatrix4x4::scale(s) * FMatrix4x4::rotation(FQuaternion::fromEulerAngles(r));
    Assert::True(m.extractDirectScale().almostEquals(s),
        "Failed to extract similar scale to input: {m.ExtractDirectScale()} != {s}");
};

void DecomposeScaleTest(float sx, float sy, float sz)
{
    auto m = FMatrix4x4::scale(sx, sy, sz);

    auto expectedScales = FVector3(sx, sy, sz);
    FVector3 scales;
    FQuaternion rotation;
    FVector3 translation;

    auto actualResult = FMatrix4x4::decompose(m, scales, rotation, translation);
    Assert::True(actualResult, "FMatrix4x4::Decompose did not return expected value.1");
    Assert::True(MathHelper::Equal(expectedScales, scales), "FMatrix4x4::Decompose did not return expected value.2");
    Assert::True(MathHelper::EqualRotation(FQuaternion::identity(), rotation), "FMatrix4x4::Decompose did not return expected value.3");
    Assert::True(MathHelper::Equal(FVector3::zero(), translation), "FMatrix4x4::Decompose did not return expected value.4");
};

void CreateReflectionTest(const FPlane& plane, const FMatrix4x4& expected)
{
    auto actual = FMatrix4x4::reflection(plane);
    Assert::True(MathHelper::Equal(actual, expected), "FMatrix4x4::reflection did not return expected value.");
    Assert::True(actual.isReflection(), "FMatrix4x4::IsReflection did not return expected value.");
};

void CreateConstrainedBillboardFact(const FVector3& placeDirection, const FVector3& rotateAxis, const FMatrix4x4& expectedRotation)
{
    FVector3 cameraPosition = FVector3(3.0f, 4.0f, 5.0f);
    auto objectPosition = cameraPosition + placeDirection * 10.0f;
    auto expected = expectedRotation * FMatrix4x4::translation(objectPosition);
    auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
    Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");

    // When you move camera along rotateAxis, result must be same.
    cameraPosition += rotateAxis * 10.0f;
    actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
    Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");

    cameraPosition -= rotateAxis * 30.0f;
    actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
    Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
};

void CreateBillboardFact(const FVector3& placeDirection, const FVector3& cameraUpVector, const FMatrix4x4& expectedRotation)
{
    auto cameraPosition = FVector3(3.0f, 4.0f, 5.0f);
    auto objectPosition = cameraPosition + placeDirection * 10.0f;
    auto expected = expectedRotation * FMatrix4x4::translation(objectPosition);
    auto actual = FMatrix4x4::billboard(objectPosition, cameraPosition, cameraUpVector, FVector3(0, 0, -1));
    Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateBillboard did not return the expected value.");
}
