#include "test_vim_math3d.h"

int main()
{
#pragma region Matrix4x4Tests
    std::cout << "Matrix4x4 Tests" << std::endl;

    // A test for Identity
    test("Matrix4x4IdentityTest", [] ()
    {
        auto val = FMatrix4x4();
        val.M11 = val.M22 = val.M33 = val.M44 = 1.0f;

        Assert::True(MathHelper::Equal(val, FMatrix4x4::identity()), "FMatrix4x4::Indentity was not set correctly.");
    });

    // A test for Determinant   
    test("Matrix4x4DeterminantTest", [] ()
    {
        auto target =
                FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
                FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
                FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));

        auto val = 1.0f;
        auto det = target.getDeterminant();

        Assert::True(MathHelper::Equal(val, det), "FMatrix4x4::Determinant was not set correctly.");
    });

    // A test for Determinant
    // Determinant test |A| = 1 / |A'|  
    test("Matrix4x4DeterminantTest1", [] ()
    {
        auto a = FMatrix4x4();
        a.M11 = 5.0f;
        a.M12 = 2.0f;
        a.M13 = 8.25f;
        a.M14 = 1.0f;
        a.M21 = 12.0f;
        a.M22 = 6.8f;
        a.M23 = 2.14f;
        a.M24 = 9.6f;
        a.M31 = 6.5f;
        a.M32 = 1.0f;
        a.M33 = 3.14f;
        a.M34 = 2.22f;
        a.M41 = 0;
        a.M42 = 0.86f;
        a.M43 = 4.0f;
        a.M44 = 1.0f;

        auto inv = a.invert();
        Assert::True(inv.has_value());

        auto detA = a.getDeterminant();
        auto detI = inv.value().getDeterminant();
        auto t = 1.0f / detI;

        // only accurate to 3 precision
        Assert::True(std::fabs(detA - t) < 1e-3, "FMatrix4x4::Determinant was not set correctly.");
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertTest", [] ()
    {
        auto mtx =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));

        auto expected = FMatrix4x4();
        expected.M11 = 0.74999994f;
        expected.M12 = -0.216506317f;
        expected.M13 = 0.62499994f;
        expected.M14 = 0.0f;

        expected.M21 = 0.433012635f;
        expected.M22 = 0.87499994f;
        expected.M23 = -0.216506317f;
        expected.M24 = 0.0f;

        expected.M31 = -0.49999997f;
        expected.M32 = 0.433012635f;
        expected.M33 = 0.74999994f;
        expected.M34 = 0.0f;

        expected.M41 = 0.0f;
        expected.M42 = 0.0f;
        expected.M43 = 0.0f;
        expected.M44 = 0.99999994f;

        auto inv = mtx.invert();
        auto actual = inv.value();

        Assert::True(inv.has_value());
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::Invert did not return the expected value.");

        // Make sure M*M is identity matrix
        auto i = mtx * actual;
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()), "FMatrix4x4::Invert did not return the expected value.");
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertIdentityTest", [] ()
    {
        auto mtx = FMatrix4x4::identity();
        auto inv = mtx.invert();
        
        Assert::True(inv.has_value());
        Assert::True(MathHelper::Equal(inv.value(), FMatrix4x4::identity()));
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertTranslationTest", [] ()
    {
        auto mtx = FMatrix4x4::translation(23, 42, 666);
        auto inv = mtx.invert();
        
        Assert::True(inv.has_value());

        auto i = mtx * inv.value();
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()));
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertRotationTest", [] ()
    {
        auto mtx = FMatrix4x4::fromYawPitchRoll(3, 4, 5);
        auto inv = mtx.invert();

        Assert::True(inv.has_value());

        auto i = mtx * inv.value();
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()));
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertScaleTest", [] ()
    {
        auto mtx = FMatrix4x4::scale(23, 42, -666);
        auto inv = mtx.invert();

        Assert::True(inv.has_value());

        auto i = mtx * inv.value();
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()));
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertProjectionTest", [] ()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(1, 1.333f, 0.1f, 666);
        auto inv = mtx.invert();

        Assert::True(inv.has_value());

        auto i = mtx * inv.value();
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()));
    });

    // A test for Invert (FMatrix4x4)
    test("Matrix4x4InvertAffineTest", [] ()
    {
        auto mtx = FMatrix4x4::fromYawPitchRoll(3, 4, 5) *
                        FMatrix4x4::scale(23, 42, -666) *
                        FMatrix4x4::translation(17, 53, 89);
        auto inv = mtx.invert();

        Assert::True(inv.has_value());

        auto i = mtx * inv.value();
        Assert::True(MathHelper::Equal(i, FMatrix4x4::identity()));
    });

    // Various rotation decompose test.
    test("Matrix4x4DecomposeTest01", [] ()
    {
        DecomposeTest(10.0f, 20.0f, 30.0f, FVector3(10, 20, 30), FVector3(2, 3, 4));

        const float step = 35.0f;

        for (auto yawAngle = -720.0f; yawAngle <= 720.0f; yawAngle += step)
        {
            for (auto pitchAngle = -720.0f; pitchAngle <= 720.0f; pitchAngle += step)
            {
                for (auto rollAngle = -720.0f; rollAngle <= 720.0f; rollAngle += step)
                {
                    DecomposeTest(yawAngle, pitchAngle, rollAngle, FVector3(10, 20, 30), FVector3(2, 3, 4));
                }
            }
        }
    });

    // Various scaled matrix decompose test.
    test("Matrix4x4DecomposeTest02", [] ()
    {
        DecomposeTest(10.0f, 20.0f, 30.0f, FVector3(10, 20, 30), FVector3(2, 3, 4));

        // Various scales.
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(1, 2, 3));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(1, 3, 2));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(2, 1, 3));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(2, 3, 1));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(3, 1, 2));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(3, 2, 1));

        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(-2, 1, 1));

        // Small scales.
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(1e-4f, 2e-4f, 3e-4f));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(1e-4f, 3e-4f, 2e-4f));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(2e-4f, 1e-4f, 3e-4f));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(2e-4f, 3e-4f, 1e-4f));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(3e-4f, 1e-4f, 2e-4f));
        DecomposeTest(0, 0, 0, FVector3::zero(), FVector3(3e-4f, 2e-4f, 1e-4f));

        // Zero scales.
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(0, 0, 0));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, 0, 0));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(0, 1, 0));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(0, 0, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(0, 1, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, 0, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, 1, 0));

        // Negative scales.
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(-1, -1, -1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, -1, -1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(-1, 1, -1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(-1, -1, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(-1, 1, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, -1, 1));
        DecomposeTest(0, 0, 0, FVector3(10, 20, 30), FVector3(1, 1, -1));
    });

    // Tiny scale decompose test.
    test("Matrix4x4DecomposeTest03", [] ()
    {
        DecomposeScaleTest(1, 2e-4f, 3e-4f);
        DecomposeScaleTest(1, 3e-4f, 2e-4f);
        DecomposeScaleTest(2e-4f, 1, 3e-4f);
        DecomposeScaleTest(2e-4f, 3e-4f, 1);
        DecomposeScaleTest(3e-4f, 1, 2e-4f);
        DecomposeScaleTest(3e-4f, 2e-4f, 1);
    });

    // Simple scale extraction test.
    test("Matrix4x4ExtractScaleTest", [] ()
    {
        ExtractScaleTest(FVector3(1, 2, 1), FVector3::zero());
        ExtractScaleTest(FVector3(-1, 2, 1), FVector3::zero());
        ExtractScaleTest(FVector3(-1, 2, -1), FVector3::zero());

        ExtractScaleTest(FVector3(1, 2, 0.75f), FVector3::unitX());
        ExtractScaleTest(FVector3(1, 2, 0.75f), FVector3::unitY());
        ExtractScaleTest(FVector3(1, 2, 0.75f), FVector3::unitZ());

        ExtractScaleTest(FVector3(1, 2, 0.75f), -FVector3::unitX());
        ExtractScaleTest(FVector3(1, 2, 0.75f), -FVector3::unitY());
        ExtractScaleTest(FVector3(1, 2, 0.75f), -FVector3::unitZ());

        ExtractScaleTest(FVector3(-1, 2, 0.75f), -FVector3::unitX());
        ExtractScaleTest(FVector3(1, -2, -0.75f), -FVector3::unitY());
        ExtractScaleTest(FVector3(1, 2, -0.75f), -FVector3::unitZ());

        // Note, for more complex rotations the extraction will not return the same scale
        // These scenarios could potentially be handled by figuring out which of the
        // axis are still in the same RH configuration, but that is a bit beyond the current scope
        // and would be better handled by a full decomposition.
        //ExtractScaleTest(FVector3(1, 2, 0.75f), FVector3(.5f, 0.3f, 1.75f));
    });
 
    test("Matrix4x4DecomposeTest04", [] ()
    {
        FVector3 scales;
        FQuaternion rotation;
        FVector3 translation;

        Assert::False(FMatrix4x4::decompose(GenerateMatrixNumberFrom1To16(), scales, rotation, translation), "decompose should have failed.");
    });

    // Transform by quaternion test
    test("Matrix4x4TransformTest", [] ()
    {
        auto target = GenerateMatrixNumberFrom1To16();
        auto m =
            FMatrix4x4::rotationX(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationY(MathHelper::ToRadians(30.0f)) *
            FMatrix4x4::rotationZ(MathHelper::ToRadians(30.0f));

        auto q = m.quaternion();

        auto expected = target * m;
        FMatrix4x4 actual;
        actual = target.transform(q);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::Transform did not return the expected value.");
    });

    // A test for CreateRotationX (float) 
    test("Matrix4x4CreateRotationXTest", [] ()
    {
        auto radians = MathHelper::ToRadians(30.0f);
        auto expected = FMatrix4x4();

        expected.M11 = 1.0f;
        expected.M22 = 0.8660254f;
        expected.M23 = 0.5f;
        expected.M32 = -0.5f;
        expected.M33 = 0.8660254f;
        expected.M44 = 1.0f;

        FMatrix4x4 actual;

        actual = FMatrix4x4::rotationX(radians);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::rotationX did not return the expected value.");
    });

    // A test for CreateRotationX (float)
    // CreateRotationX of zero degree 
    test("Matrix4x4CreateRotationXTest1", [] ()
    {
        float radians = 0;

        auto expected = FMatrix4x4::identity();
        auto actual = FMatrix4x4::rotationX(radians);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::rotationX did not return the expected value.");
    });

    // A test for CreateRotationX (float, Vector3f) 
    test("Matrix4x4CreateRotationXCenterTest", [] ()
    {
        auto radians = MathHelper::ToRadians(30.0f);
        auto center = FVector3(23, 42, 66);

        auto rotateAroundZero = FMatrix4x4::rotationX(radians, FVector3::zero());
        auto rotateAroundZeroExpected = FMatrix4x4::rotationX(radians);
        Assert::True(MathHelper::Equal(rotateAroundZero, rotateAroundZeroExpected));

        auto rotateAroundCenter = FMatrix4x4::rotationX(radians, center);
        auto rotateAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::rotationX(radians) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(rotateAroundCenter, rotateAroundCenterExpected));
    });

    // A test for CreateRotationY (float)
    test("Matrix4x4CreateRotationYTest", [] ()
    {
        auto radians = MathHelper::ToRadians(60.0f);

        auto expected = FMatrix4x4();

        expected.M11 = 0.49999997f;
        expected.M13 = -0.866025448f;
        expected.M22 = 1.0f;
        expected.M31 = 0.866025448f;
        expected.M33 = 0.49999997f;
        expected.M44 = 1.0f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::rotationY(radians);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::rotationY did not return the expected value.");
    });

    // A test for RotationY (float)
    // CreateRotationY test for negative angle
    test("Matrix4x4CreateRotationYTest1", [] ()
    {
        auto radians = MathHelper::ToRadians(-300.0f);

        auto expected = FMatrix4x4();

        expected.M11 = 0.49999997f;
        expected.M13 = -0.866025448f;
        expected.M22 = 1.0f;
        expected.M31 = 0.866025448f;
        expected.M33 = 0.49999997f;
        expected.M44 = 1.0f;

        auto actual = FMatrix4x4::rotationY(radians);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::rotationY did not return the expected value.");
    });

    // A test for CreateRotationY (float, Vector3f)
    test("Matrix4x4CreateRotationYCenterTest", [] ()
    {
        auto radians = MathHelper::ToRadians(30.0f);
        auto center = FVector3(23, 42, 66);

        auto rotateAroundZero = FMatrix4x4::rotationY(radians, FVector3::zero());
        auto rotateAroundZeroExpected = FMatrix4x4::rotationY(radians);
        Assert::True(MathHelper::Equal(rotateAroundZero, rotateAroundZeroExpected));

        auto rotateAroundCenter = FMatrix4x4::rotationY(radians, center);
        auto rotateAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::rotationY(radians) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(rotateAroundCenter, rotateAroundCenterExpected));
    });

    // A test for CreateFromAxisAngle(Vector3f,float)
    test("Matrix4x4CreateFromAxisAngleTest", [] ()
    {
        auto radians = MathHelper::ToRadians(-30.0f);

        auto expected = FMatrix4x4::rotationX(radians);
        auto actual = FMatrix4x4::fromAxisAngle(FVector3::unitX(), radians);
        Assert::True(MathHelper::Equal(expected, actual));

        expected = FMatrix4x4::rotationY(radians);
        actual = FMatrix4x4::fromAxisAngle(FVector3::unitY(), radians);
        Assert::True(MathHelper::Equal(expected, actual));

        expected = FMatrix4x4::rotationZ(radians);
        actual = FMatrix4x4::fromAxisAngle(FVector3::unitZ(), radians);
        Assert::True(MathHelper::Equal(expected, actual));

        expected = FMatrix4x4::fromQuaternion(FQuaternion::fromAxisAngle(FVector3::one().normalize(), radians));
        actual = FMatrix4x4::fromAxisAngle(FVector3::one().normalize(), radians);
        Assert::True(MathHelper::Equal(expected, actual));

        const int rotCount = 16;
        for (auto i = 0; i < rotCount; ++i)
        {
            auto latitude = (2.0f * MathHelper::pi) * (i / (float)rotCount);
            for (auto j = 0; j < rotCount; ++j)
            {
                auto longitude = -MathHelper::piOver2 + MathHelper::pi * (j / (float)rotCount);

                auto m = FMatrix4x4::rotationZ(longitude) * FMatrix4x4::rotationY(latitude);
                auto axis = FVector3(m.M11, m.M12, m.M13);
                for (auto k = 0; k < rotCount; ++k)
                {
                    auto rot = (2.0f * MathHelper::pi) * (k / (float)rotCount);
                    expected = FMatrix4x4::fromQuaternion(FQuaternion::fromAxisAngle(axis, rot));
                    actual = FMatrix4x4::fromAxisAngle(axis, rot);
                    Assert::True(MathHelper::Equal(expected, actual));
                }
            }
        }
    });

    test("Matrix4x4CreateFromYawPitchRollTest1", [] ()
    {
        auto yawAngle = MathHelper::ToRadians(30.0f);
        auto pitchAngle = MathHelper::ToRadians(40.0f);
        auto rollAngle = MathHelper::ToRadians(50.0f);

        auto yaw = FMatrix4x4::fromAxisAngle(FVector3::unitY(), yawAngle);
        auto pitch = FMatrix4x4::fromAxisAngle(FVector3::unitX(), pitchAngle);
        auto roll = FMatrix4x4::fromAxisAngle(FVector3::unitZ(), rollAngle);

        auto expected = roll * pitch * yaw;
        auto actual = FMatrix4x4::fromYawPitchRoll(yawAngle, pitchAngle, rollAngle);
        Assert::True(MathHelper::Equal(expected, actual));
    });

    // Covers more numeric rigions
    test("Matrix4x4CreateFromYawPitchRollTest2", [] ()
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
                    auto yaw = FMatrix4x4::fromAxisAngle(FVector3::unitY(), yawRad);
                    auto pitch = FMatrix4x4::fromAxisAngle(FVector3::unitX(), pitchRad);
                    auto roll = FMatrix4x4::fromAxisAngle(FVector3::unitZ(), rollRad);

                    auto expected = roll * pitch * yaw;
                    auto actual = FMatrix4x4::fromYawPitchRoll(yawRad, pitchRad, rollRad);
                    Assert::True(MathHelper::Equal(expected, actual));
                    //Assert::True(MathHelper::Equal(expected, actual), string.Format("Yaw:{0} Pitch:{1} Roll:{2}", yawAngle, pitchAngle, rollAngle));
                }
            }
        }
    });

    // Simple shadow test.
    test("Matrix4x4CreateShadowTest01", [] ()
    {
        auto lightDir = FVector3::unitY();
        auto plane = FPlane(FVector3::unitY(), 0);

        auto expected = FMatrix4x4::scale(1, 0, 1);

        auto actual = FMatrix4x4::shadow(lightDir, plane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::shadow did not returned expected value.");
    });

    // Various plane projections.
    test("Matrix4x4CreateShadowTest02", [] ()
    {
        // Complex cases.
        FPlane planes[5] = {
            FPlane( 0, 1, 0, 0 ),
            FPlane( 1, 2, 3, 4 ),
            FPlane( 5, 6, 7, 8 ),
            FPlane(-1,-2,-3,-4 ),
            FPlane(-5,-6,-7,-8 ),
        };

        FVector3 points[6] = {
            FVector3( 1, 2, 3),
            FVector3( 5, 6, 7),
            FVector3( 8, 9, 10),
            FVector3(-1,-2,-3),
            FVector3(-5,-6,-7),
            FVector3(-8,-9,-10),
        };
        
        for (const FPlane& p : planes) {
            auto plane = FPlane::normalize(p);

            // Try various direction of light directions.
            FVector3 testDirections[27] =
            {
                FVector3( -1.0f, 1.0f, 1.0f ),
                FVector3(  0.0f, 1.0f, 1.0f ),
                FVector3(  1.0f, 1.0f, 1.0f ),
                FVector3( -1.0f, 0.0f, 1.0f ),
                FVector3(  0.0f, 0.0f, 1.0f ),
                FVector3(  1.0f, 0.0f, 1.0f ),
                FVector3( -1.0f,-1.0f, 1.0f ),
                FVector3(  0.0f,-1.0f, 1.0f ),
                FVector3(  1.0f,-1.0f, 1.0f ),

                FVector3( -1.0f, 1.0f, 0.0f ),
                FVector3(  0.0f, 1.0f, 0.0f ),
                FVector3(  1.0f, 1.0f, 0.0f ),
                FVector3( -1.0f, 0.0f, 0.0f ),
                FVector3(  0.0f, 0.0f, 0.0f ),
                FVector3(  1.0f, 0.0f, 0.0f ),
                FVector3( -1.0f,-1.0f, 0.0f ),
                FVector3(  0.0f,-1.0f, 0.0f ),
                FVector3(  1.0f,-1.0f, 0.0f ),

                FVector3( -1.0f, 1.0f,-1.0f ),
                FVector3(  0.0f, 1.0f,-1.0f ),
                FVector3(  1.0f, 1.0f,-1.0f ),
                FVector3( -1.0f, 0.0f,-1.0f ),
                FVector3(  0.0f, 0.0f,-1.0f ),
                FVector3(  1.0f, 0.0f,-1.0f ),
                FVector3( -1.0f,-1.0f,-1.0f ),
                FVector3(  0.0f,-1.0f,-1.0f ),
                FVector3(  1.0f,-1.0f,-1.0f ),
            };

            for (const FVector3& lightDirInfo : testDirections)
            {
                if (lightDirInfo.length() < 0.1f)
                    continue;
                auto lightDir = lightDirInfo.normalize();

                if (FPlane::dotNormal(plane, lightDir) < 0.1f)
                    continue;

                auto m = FMatrix4x4::shadow(lightDir, plane);
                auto pp = -plane.D * plane.Normal; // origin of the plane.

                //
                for (const auto& point : points)
                {
                    auto v4 = FMatrix4x4::transformToVector4(point, m);

                    auto sp = FVector3(v4.X, v4.Y, v4.Z) / v4.W;

                    // Make sure transformed position is on the plane.
                    auto v = sp - pp;
                    auto d = FVector3::dot(v, plane.Normal);
                    Assert::True(mathOps::almostZero(d, 0.0001f), "FMatrix4x4::shadow did not provide expected value.1");

                    // make sure direction between transformed position and original position are same as light direction.
                    if ((point - pp).dot(plane.Normal) > 0.0001f)
                    {
                        auto dir = (point - sp).normalize();
                        Assert::True(MathHelper::Equal(dir, lightDir), "FMatrix4x4::shadow did not provide expected value.2");
                    }
                }
            }
        }
    }); 
        
    test("Matrix4x4CreateReflectionTest01", [] ()
    {
        // XY plane.
        CreateReflectionTest(FPlane(FVector3::unitZ(), 0), FMatrix4x4::scale(1, 1, -1));
        // XZ plane.
        CreateReflectionTest(FPlane(FVector3::unitY(), 0), FMatrix4x4::scale(1, -1, 1));
        // YZ plane.
        CreateReflectionTest(FPlane(FVector3::unitX(), 0), FMatrix4x4::scale(-1, 1, 1));

        // Complex cases.
        FPlane planes[5] = {
            FPlane( 0, 1, 0, 0 ),
            FPlane( 1, 2, 3, 4 ),
            FPlane( 5, 6, 7, 8 ),
            FPlane(-1,-2,-3,-4 ),
            FPlane(-5,-6,-7,-8 )
        };

        FVector3 points[4] = {
            FVector3( 1, 2, 3),
            FVector3( 5, 6, 7),
            FVector3(-1,-2,-3),
            FVector3(-5,-6,-7)
        };

        for (const auto& p : planes)
        {
            auto plane = FPlane::normalize(p);
            auto m = FMatrix4x4::reflection(plane);
            auto pp = -plane.D * plane.Normal; // Position on the plane.

            //
            for (const auto& point : points)
            {
                auto rp = FMatrix4x4::transform(point, m);

                // Manually compute reflection point and compare results.
                auto v = point - pp;
                auto d = v.dot(plane.Normal);
                auto vp = point - 2.0f * d * plane.Normal;
                Assert::True(MathHelper::Equal(rp, vp), "FMatrix4x4::Reflection did not provide expected value.");
            }
        }
    });

    // A test for CreateRotationZ (float)
    test("Matrix4x4CreateRotationZTest", [] ()
    {
        auto radians = MathHelper::ToRadians(50.0f);

        auto expected = FMatrix4x4();
        expected.M11 = 0.642787635f;
        expected.M12 = 0.766044438f;
        expected.M21 = -0.766044438f;
        expected.M22 = 0.642787635f;
        expected.M33 = 1.0f;
        expected.M44 = 1.0f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::rotationZ(radians);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::rotationZ did not return the expected value.");
    });

    // A test for CreateRotationZ (float, Vector3f)
    test("Matrix4x4CreateRotationZCenterTest", [] ()
    {
        auto radians = MathHelper::ToRadians(30.0f);
        auto center = FVector3(23, 42, 66);

        auto rotateAroundZero = FMatrix4x4::rotationZ(radians, FVector3::zero());
        auto rotateAroundZeroExpected = FMatrix4x4::rotationZ(radians);
        Assert::True(MathHelper::Equal(rotateAroundZero, rotateAroundZeroExpected));

        auto rotateAroundCenter = FMatrix4x4::rotationZ(radians, center);
        auto rotateAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::rotationZ(radians) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(rotateAroundCenter, rotateAroundCenterExpected));
    });

    // A test for CrateLookAt (Vector3f, Vector3f, Vector3f)
    test("Matrix4x4CreateLookAtTest", [] ()
    {
        auto cameraPosition = FVector3(10.0f, 20.0f, 30.0f);
        auto cameraTarget = FVector3(3.0f, 2.0f, -4.0f);
        auto cameraUpVector = FVector3(0.0f, 1.0f, 0.0f);

        auto expected = FMatrix4x4();
        expected.M11 = 0.979457f;
        expected.M12 = -0.0928267762f;
        expected.M13 = 0.179017f;

        expected.M21 = 0.0f;
        expected.M22 = 0.8877481f;
        expected.M23 = 0.460329473f;

        expected.M31 = -0.201652914f;
        expected.M32 = -0.450872928f;
        expected.M33 = 0.8695112f;

        expected.M41 = -3.74498272f;
        expected.M42 = -3.30050683f;
        expected.M43 = -37.0820961f;
        expected.M44 = 1.0f;

        auto actual = FMatrix4x4::lookAt(cameraPosition, cameraTarget, cameraUpVector);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateLookAt did not return the expected value.");
    });

    // A test for CreateWorld (Vector3f, Vector3f, Vector3f)
    test("Matrix4x4CreateWorldTest", [] ()
    {
        auto objectPosition = FVector3(10.0f, 20.0f, 30.0f);
        auto objectForwardDirection = FVector3(3.0f, 2.0f, -4.0f);
        auto objectUpVector = FVector3(0.0f, 1.0f, 0.0f);

        auto expected = FMatrix4x4();
        expected.M11 = 0.799999952f;
        expected.M12 = 0;
        expected.M13 = 0.599999964f;
        expected.M14 = 0;

        expected.M21 = -0.2228344f;
        expected.M22 = 0.928476632f;
        expected.M23 = 0.297112525f;
        expected.M24 = 0;

        expected.M31 = -0.557086f;
        expected.M32 = -0.371390671f;
        expected.M33 = 0.742781341f;
        expected.M34 = 0;

        expected.M41 = 10;
        expected.M42 = 20;
        expected.M43 = 30;
        expected.M44 = 1.0f;

        auto actual = FMatrix4x4::world(objectPosition, objectForwardDirection, objectUpVector);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateWorld did not return the expected value.");

        Assert::True(objectPosition == actual.translation());
        Assert::True(FVector3::dot(objectUpVector.normalize(), FVector3(actual.M21, actual.M22, actual.M23)) > 0);
        Assert::True(FVector3::dot(objectForwardDirection.normalize(), FVector3(-actual.M31, -actual.M32, -actual.M33)) > 0.999f);
    });

    // A test for CreateOrtho (float, float, float, float)
    test("Matrix4x4CreateOrthoTest", [] ()
    {
        auto width = 100.0f;
        auto height = 200.0f;
        auto zNearPlane = 1.5f;
        auto zFarPlane = 1000.0f;

        auto expected = FMatrix4x4();
        expected.M11 = 0.02f;
        expected.M22 = 0.01f;
        expected.M33 = -0.00100150227f;
        expected.M43 = -0.00150225335f;
        expected.M44 = 1.0f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::orthographic(width, height, zNearPlane, zFarPlane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateOrtho did not return the expected value.");
    });

    // A test for CreateOrthoOffCenter (float, float, float, float, float, float)
    test("Matrix4x4CreateOrthoOffCenterTest", [] ()
    {
        auto left = 10.0f;
        auto right = 90.0f;
        auto bottom = 20.0f;
        auto top = 180.0f;
        auto zNearPlane = 1.5f;
        auto zFarPlane = 1000.0f;

        auto expected = FMatrix4x4();
        expected.M11 = 0.025f;
        expected.M22 = 0.0125f;
        expected.M33 = -0.00100150227f;
        expected.M41 = -1.25f;
        expected.M42 = -1.25f;
        expected.M43 = -0.00150225335f;
        expected.M44 = 1.0f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::orthographicOffCenter(left, right, bottom, top, zNearPlane, zFarPlane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateOrthoOffCenter did not return the expected value.");
    });

    // A test for CreatePerspective (float, float, float, float)
    test("Matrix4x4CreatePerspectiveTest", [] ()
    {
        auto width = 100.0f;
        auto height = 200.0f;
        auto zNearPlane = 1.5f;
        auto zFarPlane = 1000.0f;

        auto expected = FMatrix4x4();
        expected.M11 = 0.03f;
        expected.M22 = 0.015f;
        expected.M33 = -1.00150228f;
        expected.M34 = -1.0f;
        expected.M43 = -1.50225341f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::perspective(width, height, zNearPlane, zFarPlane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::perspective did not return the expected value.");
    });

    // A test for CreatePerspective (float, float, float, float)
    // CreatePerspective test where znear = zfar
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveTest", []() {
        auto width = 100.0f;
        auto height = 200.0f;
        auto zNearPlane = 0.0f;
        auto zFarPlane = 0.0f;

        auto actual = FMatrix4x4::perspective(width, height, zNearPlane, zFarPlane);
    });

    // A test for CreatePerspective (float, float, float, float)
// CreatePerspective test where near plane is negative value
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveTest2", []()
    {
        auto actual = FMatrix4x4::perspective(10, 10, -10, 10);
    });

    // A test for CreatePerspective (float, float, float, float)
    // CreatePerspective test where far plane is negative value
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveTest3", []()
    {
        auto actual = FMatrix4x4::perspective(10, 10, 10, -10);
    });

    // A test for CreatePerspective (float, float, float, float)
    // CreatePerspective test where near plane is beyond far plane
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveTest4", []()
    {
        auto actual = FMatrix4x4::perspective(10, 10, 10, 1);
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    test("Matrix4x4CreatePerspectiveFieldOfViewTest", [] ()
    {
        auto fieldOfView = MathHelper::ToRadians(30.0f);
        auto aspectRatio = 1280.0f / 720.0f;
        auto zNearPlane = 1.5f;
        auto zFarPlane = 1000.0f;

        auto expected = FMatrix4x4();
        expected.M11 = 2.09927845f;
        expected.M22 = 3.73205066f;
        expected.M33 = -1.00150228f;
        expected.M34 = -1.0f;
        expected.M43 = -1.50225341f;
        FMatrix4x4 actual;

        actual = FMatrix4x4::perspectiveFieldOfView(fieldOfView, aspectRatio, zNearPlane, zFarPlane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreatePerspectiveFieldOfView did not return the expected value.");
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    // CreatePerspectiveFieldOfView test where filedOfView is negative value.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveFieldOfViewTest1", []()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(-1, 1, 1, 10);
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    // CreatePerspectiveFieldOfView test where filedOfView is more than pi.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveFieldOfViewTest2", []()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(MathHelper::pi + 0.01f, 1, 1, 10);
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    // CreatePerspectiveFieldOfView test where nearPlaneDistance is negative value.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveFieldOfViewTest3", []()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(MathHelper::piOver4, 1, -1, 10);
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    // CreatePerspectiveFieldOfView test where farPlaneDistance is negative value.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveFieldOfViewTest4", []()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(MathHelper::piOver4, 1, 1, -10);
    });

    // A test for CreatePerspectiveFieldOfView (float, float, float, float)
    // CreatePerspectiveFieldOfView test where nearPlaneDistance is larger than farPlaneDistance.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveFieldOfViewTest5", []()
    {
        auto mtx = FMatrix4x4::perspectiveFieldOfView(MathHelper::piOver4, 1, 10, 1);
    });

    // A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
    test("Matrix4x4CreatePerspectiveOffCenterTest", [] ()
    {
        auto left = 10.0f;
        auto right = 90.0f;
        auto bottom = 20.0f;
        auto top = 180.0f;
        auto zNearPlane = 1.5f;
        auto zFarPlane = 1000.0f;

        auto expected = FMatrix4x4();
        expected.M11 = 0.0375f;
        expected.M22 = 0.01875f;
        expected.M31 = 1.25f;
        expected.M32 = 1.25f;
        expected.M33 = -1.00150228f;
        expected.M34 = -1.0f;
        expected.M43 = -1.50225341f;

        FMatrix4x4 actual;
        actual = FMatrix4x4::perspectiveOffCenter(left, right, bottom, top, zNearPlane, zFarPlane);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreatePerspectiveOffCenter did not return the expected value.");
    });

    // A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
    // CreatePerspectiveOffCenter test where nearPlaneDistance is negative.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveOffCenterTest1", []()
    {
        float left = 10.0f, right = 90.0f, bottom = 20.0f, top = 180.0f;
        auto actual = FMatrix4x4::perspectiveOffCenter(left, right, bottom, top, -1, 10);
    });

    // A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
    // CreatePerspectiveOffCenter test where farPlaneDistance is negative.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveOffCenterTest2", []()
    {
        float left = 10.0f, right = 90.0f, bottom = 20.0f, top = 180.0f;
        auto actual = FMatrix4x4::perspectiveOffCenter(left, right, bottom, top, 1, -10);
    });

    // A test for CreatePerspectiveOffCenter (float, float, float, float, float, float)
    // CreatePerspectiveOffCenter test where test where nearPlaneDistance is larger than farPlaneDistance.
    testException<std::out_of_range>("Matrix4x4CreatePerspectiveOffCenterTest3", []()
    {
        float left = 10.0f, right = 90.0f, bottom = 20.0f, top = 180.0f;
        auto actual = FMatrix4x4::perspectiveOffCenter(left, right, bottom, top, 10, 1);
    });

    // A test for Invert (FMatrix4x4)
    // Non invertible matrix - determinant is zero - singular matrix
    test("Matrix4x4InvertTest1", [] ()
    {
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

        auto detA = a.getDeterminant();
        Assert::True(MathHelper::Equal(detA, 0.0f), "FMatrix4x4::Invert did not return the expected value.");

        auto inv = a.invert();
        Assert::False(inv.has_value());
        FMatrix4x4 actual = inv.value_or(FMatrix4x4(std::numeric_limits<float>::quiet_NaN()));

        // all the elements in Actual is NaN
        Assert::True(
            std::isnan(actual.M11) && std::isnan(actual.M12) && std::isnan(actual.M13) && std::isnan(actual.M14) &&
            std::isnan(actual.M21) && std::isnan(actual.M22) && std::isnan(actual.M23) && std::isnan(actual.M24) &&
            std::isnan(actual.M31) && std::isnan(actual.M32) && std::isnan(actual.M33) && std::isnan(actual.M34) &&
            std::isnan(actual.M41) && std::isnan(actual.M42) && std::isnan(actual.M43) && std::isnan(actual.M44)
            , "FMatrix4x4::Invert did not return the expected value.");
    });

    // A test for Lerp (FMatrix4x4, FMatrix4x4, float)
    test("Matrix4x4LerpTest", [] ()
    {
        auto a = FMatrix4x4();
        a.M11 = 11.0f;
        a.M12 = 12.0f;
        a.M13 = 13.0f;
        a.M14 = 14.0f;
        a.M21 = 21.0f;
        a.M22 = 22.0f;
        a.M23 = 23.0f;
        a.M24 = 24.0f;
        a.M31 = 31.0f;
        a.M32 = 32.0f;
        a.M33 = 33.0f;
        a.M34 = 34.0f;
        a.M41 = 41.0f;
        a.M42 = 42.0f;
        a.M43 = 43.0f;
        a.M44 = 44.0f;

        auto b = GenerateMatrixNumberFrom1To16();

        auto t = 0.5f;

        auto expected = FMatrix4x4();
        expected.M11 = a.M11 + (b.M11 - a.M11) * t;
        expected.M12 = a.M12 + (b.M12 - a.M12) * t;
        expected.M13 = a.M13 + (b.M13 - a.M13) * t;
        expected.M14 = a.M14 + (b.M14 - a.M14) * t;

        expected.M21 = a.M21 + (b.M21 - a.M21) * t;
        expected.M22 = a.M22 + (b.M22 - a.M22) * t;
        expected.M23 = a.M23 + (b.M23 - a.M23) * t;
        expected.M24 = a.M24 + (b.M24 - a.M24) * t;

        expected.M31 = a.M31 + (b.M31 - a.M31) * t;
        expected.M32 = a.M32 + (b.M32 - a.M32) * t;
        expected.M33 = a.M33 + (b.M33 - a.M33) * t;
        expected.M34 = a.M34 + (b.M34 - a.M34) * t;

        expected.M41 = a.M41 + (b.M41 - a.M41) * t;
        expected.M42 = a.M42 + (b.M42 - a.M42) * t;
        expected.M43 = a.M43 + (b.M43 - a.M43) * t;
        expected.M44 = a.M44 + (b.M44 - a.M44) * t;

        FMatrix4x4 actual;
        actual = FMatrix4x4::lerp(a, b, t);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::Lerp did not return the expected value.");
    });

    // A test for operator - (FMatrix4x4)
    test("Matrix4x4UnaryNegationTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = -1.0f;
        expected.M12 = -2.0f;
        expected.M13 = -3.0f;
        expected.M14 = -4.0f;
        expected.M21 = -5.0f;
        expected.M22 = -6.0f;
        expected.M23 = -7.0f;
        expected.M24 = -8.0f;
        expected.M31 = -9.0f;
        expected.M32 = -10.0f;
        expected.M33 = -11.0f;
        expected.M34 = -12.0f;
        expected.M41 = -13.0f;
        expected.M42 = -14.0f;
        expected.M43 = -15.0f;
        expected.M44 = -16.0f;

        auto actual = -a;
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::operator - did not return the expected value.");
    });

    // A test for operator - (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4SubtractionTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();
        auto expected = FMatrix4x4();

        auto actual = a - b;
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::operator - did not return the expected value.");
    });

    // A test for operator * (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4MultiplyTest1", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = a.M11 * b.M11 + a.M12 * b.M21 + a.M13 * b.M31 + a.M14 * b.M41;
        expected.M12 = a.M11 * b.M12 + a.M12 * b.M22 + a.M13 * b.M32 + a.M14 * b.M42;
        expected.M13 = a.M11 * b.M13 + a.M12 * b.M23 + a.M13 * b.M33 + a.M14 * b.M43;
        expected.M14 = a.M11 * b.M14 + a.M12 * b.M24 + a.M13 * b.M34 + a.M14 * b.M44;

        expected.M21 = a.M21 * b.M11 + a.M22 * b.M21 + a.M23 * b.M31 + a.M24 * b.M41;
        expected.M22 = a.M21 * b.M12 + a.M22 * b.M22 + a.M23 * b.M32 + a.M24 * b.M42;
        expected.M23 = a.M21 * b.M13 + a.M22 * b.M23 + a.M23 * b.M33 + a.M24 * b.M43;
        expected.M24 = a.M21 * b.M14 + a.M22 * b.M24 + a.M23 * b.M34 + a.M24 * b.M44;

        expected.M31 = a.M31 * b.M11 + a.M32 * b.M21 + a.M33 * b.M31 + a.M34 * b.M41;
        expected.M32 = a.M31 * b.M12 + a.M32 * b.M22 + a.M33 * b.M32 + a.M34 * b.M42;
        expected.M33 = a.M31 * b.M13 + a.M32 * b.M23 + a.M33 * b.M33 + a.M34 * b.M43;
        expected.M34 = a.M31 * b.M14 + a.M32 * b.M24 + a.M33 * b.M34 + a.M34 * b.M44;

        expected.M41 = a.M41 * b.M11 + a.M42 * b.M21 + a.M43 * b.M31 + a.M44 * b.M41;
        expected.M42 = a.M41 * b.M12 + a.M42 * b.M22 + a.M43 * b.M32 + a.M44 * b.M42;
        expected.M43 = a.M41 * b.M13 + a.M42 * b.M23 + a.M43 * b.M33 + a.M44 * b.M43;
        expected.M44 = a.M41 * b.M14 + a.M42 * b.M24 + a.M43 * b.M34 + a.M44 * b.M44;

        auto actual = a * b;
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::operator * did not return the expected value.");
    });

    // A test for operator * (FMatrix4x4, FMatrix4x4)
    // Multiply with identity matrix 
    test("Matrix4x4MultiplyTest4", [] ()
    {
        auto a = FMatrix4x4();
        a.M11 = 1.0f;
        a.M12 = 2.0f;
        a.M13 = 3.0f;
        a.M14 = 4.0f;
        a.M21 = 5.0f;
        a.M22 = -6.0f;
        a.M23 = 7.0f;
        a.M24 = -8.0f;
        a.M31 = 9.0f;
        a.M32 = 10.0f;
        a.M33 = 11.0f;
        a.M34 = 12.0f;
        a.M41 = 13.0f;
        a.M42 = -14.0f;
        a.M43 = 15.0f;
        a.M44 = -16.0f;

        auto b = FMatrix4x4();
        b = FMatrix4x4::identity();

        auto expected = a;
        auto actual = a * b;

        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::operator * did not return the expected value.");
    });

    // A test for operator + (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4AdditionTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = a.M11 + b.M11;
        expected.M12 = a.M12 + b.M12;
        expected.M13 = a.M13 + b.M13;
        expected.M14 = a.M14 + b.M14;
        expected.M21 = a.M21 + b.M21;
        expected.M22 = a.M22 + b.M22;
        expected.M23 = a.M23 + b.M23;
        expected.M24 = a.M24 + b.M24;
        expected.M31 = a.M31 + b.M31;
        expected.M32 = a.M32 + b.M32;
        expected.M33 = a.M33 + b.M33;
        expected.M34 = a.M34 + b.M34;
        expected.M41 = a.M41 + b.M41;
        expected.M42 = a.M42 + b.M42;
        expected.M43 = a.M43 + b.M43;
        expected.M44 = a.M44 + b.M44;

        FMatrix4x4 actual;

        actual = a + b;

        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::operator + did not return the expected value.");
    });

    // A test for Transpose (FMatrix4x4)
    test("Matrix4x4TransposeTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = a.M11;
        expected.M12 = a.M21;
        expected.M13 = a.M31;
        expected.M14 = a.M41;
        expected.M21 = a.M12;
        expected.M22 = a.M22;
        expected.M23 = a.M32;
        expected.M24 = a.M42;
        expected.M31 = a.M13;
        expected.M32 = a.M23;
        expected.M33 = a.M33;
        expected.M34 = a.M43;
        expected.M41 = a.M14;
        expected.M42 = a.M24;
        expected.M43 = a.M34;
        expected.M44 = a.M44;

        auto actual = FMatrix4x4::transpose(a);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::Transpose did not return the expected value.");
    });

    // A test for Transpose (FMatrix4x4)
    // Transpose Identity matrix
    test("Matrix4x4TransposeTest1", [] ()
    {
        auto a = FMatrix4x4::identity();
        auto expected = FMatrix4x4::identity();

        auto actual = FMatrix4x4::transpose(a);
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::Transpose did not return the expected value.");
    });

    // A test for FMatrix4x4 (FQuaternion) 
    test("Matrix4x4FromQuaternionTest1", [] ()
    {
        auto axis = FVector3(1.0f, 2.0f, 3.0f).normalize();
        auto q = FQuaternion::fromAxisAngle(axis, MathHelper::ToRadians(30.0f));

        auto expected = FMatrix4x4();
        expected.M11 = 0.875595033f;
        expected.M12 = 0.420031041f;
        expected.M13 = -0.2385524f;
        expected.M14 = 0.0f;

        expected.M21 = -0.38175258f;
        expected.M22 = 0.904303849f;
        expected.M23 = 0.1910483f;
        expected.M24 = 0.0f;

        expected.M31 = 0.295970082f;
        expected.M32 = -0.07621294f;
        expected.M33 = 0.952151954f;
        expected.M34 = 0.0f;

        expected.M41 = 0.0f;
        expected.M42 = 0.0f;
        expected.M43 = 0.0f;
        expected.M44 = 1.0f;

        auto target = FMatrix4x4::fromQuaternion(q);
        Assert::True(MathHelper::Equal(expected, target), "FMatrix4x4::FMatrix4x4(FQuaternion) did not return the expected value.");
    });

    // A test for FromQuaternion (FMatrix4x4)
    // Convert X axis rotation matrix 
    test("Matrix4x4FromQuaternionTest2", [] ()
    {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto quat = FQuaternion::fromAxisAngle(FVector3::unitX(), angle);

            auto expected = FMatrix4x4::rotationX(angle);
            auto actual = FMatrix4x4::fromQuaternion(quat);
            Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::FromQuaternion did not return the expected value.1");

            // make sure convert back to quaternion is same as we passed FQuaternion::
            auto q2 = actual.quaternion();
            Assert::True(MathHelper::EqualRotation(quat, q2), "FQuaternion::FromQuaternion did not return the expected value.2");
        }
    });

    // A test for FromQuaternion (FMatrix4x4)
    // Convert Y axis rotation matrix  
    test("Matrix4x4FromQuaternionTest3", [] ()
    {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto quat = FQuaternion::fromAxisAngle(FVector3::unitY(), angle);

            auto expected = FMatrix4x4::rotationY(angle);
            auto actual = FMatrix4x4::fromQuaternion(quat);
            Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::FromQuaternion did not return the expected value.1");

            // make sure convert back to quaternion is same as we passed FQuaternion::
            auto q2 = actual.quaternion();
            Assert::True(MathHelper::EqualRotation(quat, q2), "FQuaternion::FromQuaternion did not return the expected value.2");
        }
    });

    // A test for FromQuaternion (FMatrix4x4)
    // Convert Z axis rotation matrix   
    test("Matrix4x4FromQuaternionTest4", [] ()
    {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto quat = FQuaternion::fromAxisAngle(FVector3::unitZ(), angle);

            auto expected = FMatrix4x4::rotationZ(angle);
            auto actual = FMatrix4x4::fromQuaternion(quat);
            Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::FromQuaternion did not return the expected value.1");

            // make sure convert back to quaternion is same as we passed FQuaternion::
            auto q2 = actual.quaternion();
            Assert::True(MathHelper::EqualRotation(quat, q2), "FQuaternion::FromQuaternion did not return the expected value.2");
        }
    });

    // A test for FromQuaternion (FMatrix4x4)
    // Convert XYZ axis rotation matrix 
    test("Matrix4x4FromQuaternionTest5", [] ()
    {
        for (auto angle = 0.0f; angle < 720.0f; angle += 10.0f)
        {
            auto quat =
                FQuaternion::fromAxisAngle(FVector3::unitZ(), angle) *
                FQuaternion::fromAxisAngle(FVector3::unitY(), angle) *
                FQuaternion::fromAxisAngle(FVector3::unitX(), angle);

            auto expected =
                FMatrix4x4::rotationX(angle) *
                FMatrix4x4::rotationY(angle) *
                FMatrix4x4::rotationZ(angle);
            auto actual = FMatrix4x4::fromQuaternion(quat);
            Assert::True(MathHelper::Equal(expected, actual), "FQuaternion::FromQuaternion did not return the expected value.1");

            // make sure convert back to quaternion is same as we passed FQuaternion::
            auto q2 = actual.quaternion();
            Assert::True(MathHelper::EqualRotation(quat, q2), "FQuaternion::FromQuaternion did not return the expected value.2");
        }
    });

    // A test for ToString () 
    test("Matrix4x4ToStringTest", [] ()
    {
        auto a = FMatrix4x4();
        a.M11 = 11.0f;
        a.M12 = -12.0f;
        a.M13 = -13.3f;
        a.M14 = 14.4f;
        a.M21 = 21.0f;
        a.M22 = 22.0f;
        a.M23 = 23.0f;
        a.M24 = 24.0f;
        a.M31 = 31.0f;
        a.M32 = 32.0f;
        a.M33 = 33.0f;
        a.M34 = 34.0f;
        a.M41 = 41.0f;
        a.M42 = 42.0f;
        a.M43 = 43.0f;
        a.M44 = 44.0f;

        std::stringstream actualOss;
        actualOss << a;
        std::string actual = actualOss.str();

        std::ostringstream expectedlOss;
        expectedlOss << "{{ {{M11:" << a.M11 << " M12:" << a.M12 << " M13:" << a.M13 << " M14:" << a.M14 << "}} {{M21:" << a.M21 << " M22:" << a.M22 << " M23:" << a.M23 << " M24:" << a.M24 << "}} {{M31:" << a.M31 << " M32:" << a.M32 << " M33:" << a.M33 << " M34:" << a.M34 << "}} {{M41:" << a.M41 << " M42:" << a.M42 << " M43:" << a.M43 << " M44:" << a.M44 << "}}} }}";
        std::string expected = expectedlOss.str();

        Assert::True(expected == actual);
    });

    // A test for Add (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4AddTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = a.M11 + b.M11;
        expected.M12 = a.M12 + b.M12;
        expected.M13 = a.M13 + b.M13;
        expected.M14 = a.M14 + b.M14;
        expected.M21 = a.M21 + b.M21;
        expected.M22 = a.M22 + b.M22;
        expected.M23 = a.M23 + b.M23;
        expected.M24 = a.M24 + b.M24;
        expected.M31 = a.M31 + b.M31;
        expected.M32 = a.M32 + b.M32;
        expected.M33 = a.M33 + b.M33;
        expected.M34 = a.M34 + b.M34;
        expected.M41 = a.M41 + b.M41;
        expected.M42 = a.M42 + b.M42;
        expected.M43 = a.M43 + b.M43;
        expected.M44 = a.M44 + b.M44;

        FMatrix4x4 actual = a.add(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for Equals (object) 
    test("Matrix4x4EqualsTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        // case 1: compare between same values
        auto obj = b;

        bool expected = true;
        bool actual = a == obj;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b.M11 = 11.0f;
        obj = b;
        expected = false;
        actual = a == obj;
        Assert::AreEqual(expected, actual);

        //// case 3: compare between different types.
        //obj = FVector4();
        //expected = false;
        //actual = a == obj;
        //Assert::AreEqual(expected, actual);

        //// case 3: compare against null.
        //obj = null;
        //expected = false;
        //actual = a.Equals(obj);
        //Assert::AreEqual(expected, actual);
    });

    // A test for hash ()
    test("Matrix4x4hashTest", [] ()
    {
        FMatrix4x4 target = GenerateMatrixNumberFrom1To16();

        auto expected = hash::combine({ target.M11, target.M12, target.M13, target.M14, target.M21, target.M22, target.M23, target.M24, target.M31, target.M32, target.M33, target.M34, target.M41, target.M42, target.M43, target.M44 });
        auto actual = target.hash();

        Assert::True(expected == actual);
    });

    // A test for Multiply (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4MultiplyTest3", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = a.M11 * b.M11 + a.M12 * b.M21 + a.M13 * b.M31 + a.M14 * b.M41;
        expected.M12 = a.M11 * b.M12 + a.M12 * b.M22 + a.M13 * b.M32 + a.M14 * b.M42;
        expected.M13 = a.M11 * b.M13 + a.M12 * b.M23 + a.M13 * b.M33 + a.M14 * b.M43;
        expected.M14 = a.M11 * b.M14 + a.M12 * b.M24 + a.M13 * b.M34 + a.M14 * b.M44;

        expected.M21 = a.M21 * b.M11 + a.M22 * b.M21 + a.M23 * b.M31 + a.M24 * b.M41;
        expected.M22 = a.M21 * b.M12 + a.M22 * b.M22 + a.M23 * b.M32 + a.M24 * b.M42;
        expected.M23 = a.M21 * b.M13 + a.M22 * b.M23 + a.M23 * b.M33 + a.M24 * b.M43;
        expected.M24 = a.M21 * b.M14 + a.M22 * b.M24 + a.M23 * b.M34 + a.M24 * b.M44;

        expected.M31 = a.M31 * b.M11 + a.M32 * b.M21 + a.M33 * b.M31 + a.M34 * b.M41;
        expected.M32 = a.M31 * b.M12 + a.M32 * b.M22 + a.M33 * b.M32 + a.M34 * b.M42;
        expected.M33 = a.M31 * b.M13 + a.M32 * b.M23 + a.M33 * b.M33 + a.M34 * b.M43;
        expected.M34 = a.M31 * b.M14 + a.M32 * b.M24 + a.M33 * b.M34 + a.M34 * b.M44;

        expected.M41 = a.M41 * b.M11 + a.M42 * b.M21 + a.M43 * b.M31 + a.M44 * b.M41;
        expected.M42 = a.M41 * b.M12 + a.M42 * b.M22 + a.M43 * b.M32 + a.M44 * b.M42;
        expected.M43 = a.M41 * b.M13 + a.M42 * b.M23 + a.M43 * b.M33 + a.M44 * b.M43;
        expected.M44 = a.M41 * b.M14 + a.M42 * b.M24 + a.M43 * b.M34 + a.M44 * b.M44;
        FMatrix4x4 actual = a.multiply(b);

        Assert::AreEqual(expected, actual);
    });

    // A test for Multiply (FMatrix4x4, float)
    test("Matrix4x4MultiplyTest5", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto expected = FMatrix4x4(3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48);
        auto actual = a.multiply(3);

        Assert::AreEqual(expected, actual);
    });

    // A test for Multiply (FMatrix4x4, float)
    test("Matrix4x4MultiplyTest6", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto expected = FMatrix4x4(3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36, 39, 42, 45, 48);
        auto actual = a * 3;

        Assert::AreEqual(expected, actual);
    });

    // A test for Negate (FMatrix4x4)
    test("Matrix4x4NegateTest", [] ()
    {
        auto m = GenerateMatrixNumberFrom1To16();

        auto expected = FMatrix4x4();
        expected.M11 = -1.0f;
        expected.M12 = -2.0f;
        expected.M13 = -3.0f;
        expected.M14 = -4.0f;
        expected.M21 = -5.0f;
        expected.M22 = -6.0f;
        expected.M23 = -7.0f;
        expected.M24 = -8.0f;
        expected.M31 = -9.0f;
        expected.M32 = -10.0f;
        expected.M33 = -11.0f;
        expected.M34 = -12.0f;
        expected.M41 = -13.0f;
        expected.M42 = -14.0f;
        expected.M43 = -15.0f;
        expected.M44 = -16.0f;
        FMatrix4x4 actual;

        actual = m.negate();
        Assert::AreEqual(expected, actual);
    });

    // A test for operator != (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4InequalityTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        // case 1: compare between same values
        auto expected = false;
        auto actual = a != b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b.M11 = 11.0f;
        expected = true;
        actual = a != b;
        Assert::AreEqual(expected, actual);
    });

    // A test for operator == (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4EqualityTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        // case 1: compare between same values
        auto expected = true;
        auto actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b.M11 = 11.0f;
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for Subtract (FMatrix4x4, FMatrix4x4)
    test("Matrix4x4SubtractTest", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();
        auto expected = FMatrix4x4();
        FMatrix4x4 actual;

        actual = a.subtract(b);
        Assert::AreEqual(expected, actual);
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Forward side of camera on XZ-plane
    test("Matrix4x4CreateBillboardTest01", [] () {
        // Object placed at Forward of camera. result must be same as 180 degrees rotate along y-axis.
        CreateBillboardFact(FVector3(0, 0, -1), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Backward side of camera on XZ-plane
    test("Matrix4x4CreateBillboardTest02", [] () {
        // Object placed at Backward of camera. This result must be same as 0 degrees rotate along y-axis.
        CreateBillboardFact(FVector3(0, 0, 1), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(0)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Right side of camera on XZ-plane
    test("Matrix4x4CreateBillboardTest03", [] () {
        // Place object at Right side of camera. This result must be same as 90 degrees rotate along y-axis.
        CreateBillboardFact(FVector3(1, 0, 0), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(90)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Left side of camera on XZ-plane
    test("Matrix4x4CreateBillboardTest04", [] () {
        // Place object at Left side of camera. This result must be same as -90 degrees rotate along y-axis.
        CreateBillboardFact(FVector3(-1, 0, 0), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(-90)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Up side of camera on XY-plane
    test("Matrix4x4CreateBillboardTest05", [] () {
       // Place object at Up side of camera. result must be same as 180 degrees rotate along z-axis after 90 degrees rotate along x-axis.
       CreateBillboardFact(FVector3(0, 1, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(180)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Down side of camera on XY-plane
    test("Matrix4x4CreateBillboardTest06", [] () {
       // Place object at Down side of camera. result must be same as 0 degrees rotate along z-axis after 90 degrees rotate along x-axis.
       CreateBillboardFact(FVector3(0, -1, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(0)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Right side of camera on XY-plane
    test("Matrix4x4CreateBillboardTest07", [] () {
        // Place object at Right side of camera. result must be same as 90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateBillboardFact(FVector3(1, 0, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Left side of camera on XY-plane
    test("Matrix4x4CreateBillboardTest08", [] () {
        // Place object at Left side of camera. result must be same as -90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateBillboardFact(FVector3(-1, 0, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(-90.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Up side of camera on YZ-plane
    test("Matrix4x4CreateBillboardTest09", [] () {
        // Place object at Up side of camera. result must be same as -90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateBillboardFact(FVector3(0, 1, 0), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(-90.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Down side of camera on YZ-plane
    test("Matrix4x4CreateBillboardTest10", [] () {
        // Place object at Down side of camera. result must be same as 90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateBillboardFact(FVector3(0, -1, 0), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Forward side of camera on YZ-plane
    test("Matrix4x4CreateBillboardTest11", [] () {
        // Place object at Forward side of camera. result must be same as 180 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateBillboardFact(FVector3(0, 0, -1), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(180.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Backward side of camera on YZ-plane
    test("Matrix4x4CreateBillboardTest12", [] () {
        // Place object at Backward side of camera. result must be same as 0 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateBillboardFact(FVector3(0, 0, 1), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(0.0f)));
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Object and camera positions are too close and doesn't pass cameraForwardVector.
    test("Matrix4x4CreateBillboardTooCloseTest1", [] ()
    {
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto cameraPosition = objectPosition;
        auto cameraUpVector = FVector3(0, 1, 0);

        // Doesn't pass camera face direction. CreateBillboard uses Vector3f(0, 0, -1) direction. Result must be same as 180 degrees rotate along y-axis.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::billboard(objectPosition, cameraPosition, cameraUpVector, FVector3(0, 0, 1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateBillboard did not return the expected value.");
    });

    // A test for CreateBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Object and camera positions are too close and passed cameraForwardVector.
    test("Matrix4x4CreateBillboardTooCloseTest2", [] ()
    {
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto cameraPosition = objectPosition;
        auto cameraUpVector = FVector3(0, 1, 0);

        // Passes Vector3f.Right as camera face direction. Result must be same as -90 degrees rotate along y-axis.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::billboard(objectPosition, cameraPosition, cameraUpVector, FVector3(1, 0, 0));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateBillboard did not return the expected value.");
    });
 
    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Forward side of camera on XZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest01", [] () {
        // Object placed at Forward of camera. result must be same as 180 degrees rotate along y-axis.
        CreateConstrainedBillboardFact(FVector3(0, 0, -1), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Backward side of camera on XZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest02", [] () {
        // Object placed at Backward of camera. This result must be same as 0 degrees rotate along y-axis.
        CreateConstrainedBillboardFact(FVector3(0, 0, 1), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(0)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Right side of camera on XZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest03", [] () {
        // Place object at Right side of camera. This result must be same as 90 degrees rotate along y-axis.
        CreateConstrainedBillboardFact(FVector3(1, 0, 0), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(90)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Left side of camera on XZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest04", [] () {
        // Place object at Left side of camera. This result must be same as -90 degrees rotate along y-axis.
        CreateConstrainedBillboardFact(FVector3(-1, 0, 0), FVector3(0, 1, 0), FMatrix4x4::rotationY(MathHelper::ToRadians(-90)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Up side of camera on XY-plane
    test("Matrix4x4CreateConstrainedBillboardTest05", [] () {
        // Place object at Up side of camera. result must be same as 180 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateConstrainedBillboardFact(FVector3(0, 1, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(180)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Down side of camera on XY-plane
    test("Matrix4x4CreateConstrainedBillboardTest06", [] () {
        // Place object at Down side of camera. result must be same as 0 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateConstrainedBillboardFact(FVector3(0, -1, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(0)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Right side of camera on XY-plane
    test("Matrix4x4CreateConstrainedBillboardTest07", [] () {
        // Place object at Right side of camera. result must be same as 90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateConstrainedBillboardFact(FVector3(1, 0, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Left side of camera on XY-plane
    test("Matrix4x4CreateConstrainedBillboardTest08", [] () {
        // Place object at Left side of camera. result must be same as -90 degrees rotate along z-axis after 90 degrees rotate along x-axis.
        CreateConstrainedBillboardFact(FVector3(-1, 0, 0), FVector3(0, 0, 1),
            FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(-90.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Up side of camera on YZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest09", [] () {
        // Place object at Up side of camera. result must be same as -90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateConstrainedBillboardFact(FVector3(0, 1, 0), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(-90.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Down side of camera on YZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest10", [] () {
        // Place object at Down side of camera. result must be same as 90 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateConstrainedBillboardFact(FVector3(0, -1, 0), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(90.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Forward side of camera on YZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest11", [] () {
        // Place object at Forward side of camera. result must be same as 180 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateConstrainedBillboardFact(FVector3(0, 0, -1), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(180.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Place object at Backward side of camera on YZ-plane
    test("Matrix4x4CreateConstrainedBillboardTest12", [] () {
        // Place object at Backward side of camera. result must be same as 0 degrees rotate along x-axis after 90 degrees rotate along z-axis.
        CreateConstrainedBillboardFact(FVector3(0, 0, 1), FVector3(-1, 0, 0),
            FMatrix4x4::rotationZ(MathHelper::ToRadians(90.0f)) * FMatrix4x4::rotationX(MathHelper::ToRadians(0.0f)));
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Object and camera positions are too close and doesn't pass cameraForwardVector.
    test("Matrix4x4CreateConstrainedBillboardTooCloseTest1", [] ()
    {
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto cameraPosition = objectPosition;
        auto cameraUpVector = FVector3(0, 1, 0);

        // Doesn't pass camera face direction. CreateConstrainedBillboard uses Vector3f(0, 0, -1) direction. Result must be same as 180 degrees rotate along y-axis.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, cameraUpVector, FVector3(0, 0, 1), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Object and camera positions are too close and passed cameraForwardVector.
    test("Matrix4x4CreateConstrainedBillboardTooCloseTest2", [] ()
    {
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto cameraPosition = objectPosition;
        auto cameraUpVector = FVector3(0, 1, 0);

        // Passes Vector3f.Right as camera face direction. Result must be same as -90 degrees rotate along y-axis.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, cameraUpVector, FVector3(1, 0, 0), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Angle between rotateAxis and camera to object vector is too small. And use doesn't passed objectForwardVector parameter.
    test("Matrix4x4CreateConstrainedBillboardAlongAxisTest1", [] ()
    {
        // Place camera at up side of object.
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto rotateAxis = FVector3(0, 1, 0);
        auto cameraPosition = objectPosition + rotateAxis * 10.0f;

        // In this case, CreateConstrainedBillboard picks Vector3f(0, 0, -1) as object forward vector.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Angle between rotateAxis and camera to object vector is too small. And user doesn't passed objectForwardVector parameter.
    test("Matrix4x4CreateConstrainedBillboardAlongAxisTest2", [] ()
    {
        // Place camera at up side of object.
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto rotateAxis = FVector3(0, 0, -1);
        auto cameraPosition = objectPosition + rotateAxis * 10.0f;

        // In this case, CreateConstrainedBillboard picks Vector3f(1, 0, 0) as object forward vector.
        auto expected = FMatrix4x4::rotationX(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Angle between rotateAxis and camera to object vector is too small. And user passed correct objectForwardVector parameter.
    test("Matrix4x4CreateConstrainedBillboardAlongAxisTest3", [] ()
    {
        // Place camera at up side of object.
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto rotateAxis = FVector3(0, 1, 0);
        auto cameraPosition = objectPosition + rotateAxis * 10.0f;

        // User passes correct objectForwardVector.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Angle between rotateAxis and camera to object vector is too small. And user passed incorrect objectForwardVector parameter.
    test("Matrix4x4CreateConstrainedBillboardAlongAxisTest4", [] ()
    {
        // Place camera at up side of object.
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto rotateAxis = FVector3(0, 1, 0);
        auto cameraPosition = objectPosition + rotateAxis * 10.0f;

        // User passes correct objectForwardVector.
        auto expected = FMatrix4x4::rotationY(MathHelper::ToRadians(180.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 1, 0));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateConstrainedBillboard (Vector3f, Vector3f, Vector3f, Vector3f?)
    // Angle between rotateAxis and camera to object vector is too small. And user passed incorrect objectForwardVector parameter.
    test("Matrix4x4CreateConstrainedBillboardAlongAxisTest5", [] ()
    {
        // Place camera at up side of object.
        auto objectPosition = FVector3(3.0f, 4.0f, 5.0f);
        auto rotateAxis = FVector3(0, 0, -1);
        auto cameraPosition = objectPosition + rotateAxis * 10.0f;

        // In this case, CreateConstrainedBillboard picks Vector3f.Right as object forward vector.
        auto expected = FMatrix4x4::rotationX(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::rotationZ(MathHelper::ToRadians(-90.0f)) * FMatrix4x4::translation(objectPosition);
        auto actual = FMatrix4x4::constrainedBillboard(objectPosition, cameraPosition, rotateAxis, FVector3(0, 0, -1), FVector3(0, 0, -1));
        Assert::True(MathHelper::Equal(expected, actual), "FMatrix4x4::CreateConstrainedBillboard did not return the expected value.");
    });

    // A test for CreateScale (Vector3f)
    test("Matrix4x4CreateScaleTest1", [] ()
    {
        auto scales = FVector3(2.0f, 3.0f, 4.0f);
        auto expected = FMatrix4x4(
            2.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 3.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 4.0f, 0.0f,
            0.0f, 0.0f, 0.0f, 1.0f);
        auto actual = FMatrix4x4::scale(scales);
        Assert::AreEqual(expected, actual);
    });

    // A test for CreateScale (Vector3f, Vector3f)
    test("Matrix4x4CreateScaleCenterTest1", [] ()
    {
        auto scale = FVector3(3, 4, 5);
        auto center = FVector3(23, 42, 666);

        auto scaleAroundZero = FMatrix4x4::scale(scale, FVector3::zero());
        auto scaleAroundZeroExpected = FMatrix4x4::scale(scale);
        Assert::True(MathHelper::Equal(scaleAroundZero, scaleAroundZeroExpected));

        auto scaleAroundCenter = FMatrix4x4::scale(scale, center);
        auto scaleAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::scale(scale) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(scaleAroundCenter, scaleAroundCenterExpected));
    });

    // A test for CreateScale (float)  
    test("Matrix4x4CreateScaleTest2", [] ()
    {
        auto scale = 2.0f;
        auto expected = FMatrix4x4(
            2.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 2.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 2.0f, 0.0f,
            0.0f, 0.0f, 0.0f, 1.0f);
        auto actual = FMatrix4x4::scale(scale);
        Assert::AreEqual(expected, actual);
    });

    // A test for CreateScale (float, Vector3f)  
    test("Matrix4x4CreateScaleCenterTest2", [] ()
    {
        float scale = 5;
        auto center = FVector3(23, 42, 666);

        auto scaleAroundZero = FMatrix4x4::scale(scale, FVector3::zero());
        auto scaleAroundZeroExpected = FMatrix4x4::scale(scale);
        Assert::True(MathHelper::Equal(scaleAroundZero, scaleAroundZeroExpected));

        auto scaleAroundCenter = FMatrix4x4::scale(scale, center);
        auto scaleAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::scale(scale) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(scaleAroundCenter, scaleAroundCenterExpected));
    });

    // A test for CreateScale (float, float, float)
    test("Matrix4x4CreateScaleTest3", [] ()
    {
        auto xScale = 2.0f;
        auto yScale = 3.0f;
        auto zScale = 4.0f;
        auto expected = FMatrix4x4(
            2.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 3.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 4.0f, 0.0f,
            0.0f, 0.0f, 0.0f, 1.0f);
        auto actual = FMatrix4x4::scale(xScale, yScale, zScale);
        Assert::AreEqual(expected, actual);
    });

    // A test for CreateScale (float, float, float, Vector3f)
    test("Matrix4x4CreateScaleCenterTest3", [] ()
    {
        auto scale = FVector3(3, 4, 5);
        auto center = FVector3(23, 42, 666);

        auto scaleAroundZero = FMatrix4x4::scale(scale.X, scale.Y, scale.Z, FVector3::zero());
        auto scaleAroundZeroExpected = FMatrix4x4::scale(scale.X, scale.Y, scale.Z);
        Assert::True(MathHelper::Equal(scaleAroundZero, scaleAroundZeroExpected));

        auto scaleAroundCenter = FMatrix4x4::scale(scale.X, scale.Y, scale.Z, center);
        auto scaleAroundCenterExpected = FMatrix4x4::translation(-center) * FMatrix4x4::scale(scale.X, scale.Y, scale.Z) * FMatrix4x4::translation(center);
        Assert::True(MathHelper::Equal(scaleAroundCenter, scaleAroundCenterExpected));
    });

    // A test for CreateTranslation (Vector3f)
    test("Matrix4x4CreateTranslationTest1", [] ()
    {
        auto position = FVector3(2.0f, 3.0f, 4.0f);
        auto expected = FMatrix4x4(
            1.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 1.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 1.0f, 0.0f,
            2.0f, 3.0f, 4.0f, 1.0f);

        auto actual = FMatrix4x4::translation(position);
        Assert::AreEqual(expected, actual);
    });

    // A test for CreateTranslation (float, float, float)
    test("Matrix4x4CreateTranslationTest2", [] ()
    {
        auto xPosition = 2.0f;
        auto yPosition = 3.0f;
        auto zPosition = 4.0f;

        auto expected = FMatrix4x4(
            1.0f, 0.0f, 0.0f, 0.0f,
            0.0f, 1.0f, 0.0f, 0.0f,
            0.0f, 0.0f, 1.0f, 0.0f,
            2.0f, 3.0f, 4.0f, 1.0f);

        auto actual = FMatrix4x4::translation(xPosition, yPosition, zPosition);
        Assert::AreEqual(expected, actual);
    });

    // A test for Translation 
    test("Matrix4x4TranslationTest", [] ()
    {
        auto a = GenerateTestMatrix();
        auto b = a;

        // Transformed vector that has same semantics of property must be same.
        auto val = FVector3(a.M41, a.M42, a.M43);
        Assert::AreEqual(val, a.translation());

        // Set value and get value must be same.
        val = FVector3(1.0f, 2.0f, 3.0f);
        a = a.setTranslation(val);
        Assert::AreEqual(val, a.translation());

        // Make sure it only modifies expected value of matrix.
        Assert::True(
            a.M11 == b.M11 && a.M12 == b.M12 && a.M13 == b.M13 && a.M14 == b.M14 &&
            a.M21 == b.M21 && a.M22 == b.M22 && a.M23 == b.M23 && a.M24 == b.M24 &&
            a.M31 == b.M31 && a.M32 == b.M32 && a.M33 == b.M33 && a.M34 == b.M34 &&
            a.M41 != b.M41 && a.M42 != b.M42 && a.M43 != b.M43 && a.M44 == b.M44);
    });

    // A test for Equals (FMatrix4x4)  
    test("Matrix4x4EqualsTest1", [] ()
    {
        auto a = GenerateMatrixNumberFrom1To16();
        auto b = GenerateMatrixNumberFrom1To16();

        // case 1: compare between same values
        bool expected = true;
        bool actual = a == b;
        Assert::AreEqual(expected, actual);

        // case 2: compare between different values
        b.M11 = 11.0f;
        expected = false;
        actual = a == b;
        Assert::AreEqual(expected, actual);
    });

    // A test for isIdentity()  
    test("Matrix4x4isIdentityTest", [] ()
    {
        Assert::True(FMatrix4x4::identity().isIdentity());
        Assert::True(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1).isIdentity());
        Assert::False(FMatrix4x4(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0).isIdentity());
    });

    // A test for FMatrix4x4 comparison involving NaN values
    test("Matrix4x4EqualsNanTest", [] ()
    {
        auto a = FMatrix4x4(std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto b = FMatrix4x4(0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto c = FMatrix4x4(0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto d = FMatrix4x4(0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto e = FMatrix4x4(0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto f = FMatrix4x4(0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto g = FMatrix4x4(0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0, 0);
        auto h = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0, 0);
        auto i = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0, 0);
        auto j = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0, 0);
        auto k = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0, 0);
        auto l = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0, 0);
        auto m = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0, 0);
        auto n = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0, 0);
        auto o = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN(), 0);
        auto p = FMatrix4x4(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, std::numeric_limits<float>::quiet_NaN());

        Assert::False(a == FMatrix4x4());
        Assert::False(b == FMatrix4x4());
        Assert::False(c == FMatrix4x4());
        Assert::False(d == FMatrix4x4());
        Assert::False(e == FMatrix4x4());
        Assert::False(f == FMatrix4x4());
        Assert::False(g == FMatrix4x4());
        Assert::False(h == FMatrix4x4());
        Assert::False(i == FMatrix4x4());
        Assert::False(j == FMatrix4x4());
        Assert::False(k == FMatrix4x4());
        Assert::False(l == FMatrix4x4());
        Assert::False(m == FMatrix4x4());
        Assert::False(n == FMatrix4x4());
        Assert::False(o == FMatrix4x4());
        Assert::False(p == FMatrix4x4());

        Assert::True(a != FMatrix4x4());
        Assert::True(b != FMatrix4x4());
        Assert::True(c != FMatrix4x4());
        Assert::True(d != FMatrix4x4());
        Assert::True(e != FMatrix4x4());
        Assert::True(f != FMatrix4x4());
        Assert::True(g != FMatrix4x4());
        Assert::True(h != FMatrix4x4());
        Assert::True(i != FMatrix4x4());
        Assert::True(j != FMatrix4x4());
        Assert::True(k != FMatrix4x4());
        Assert::True(l != FMatrix4x4());
        Assert::True(m != FMatrix4x4());
        Assert::True(n != FMatrix4x4());
        Assert::True(o != FMatrix4x4());
        Assert::True(p != FMatrix4x4());

        Assert::False(a == (FMatrix4x4()));
        Assert::False(b == (FMatrix4x4()));
        Assert::False(c == (FMatrix4x4()));
        Assert::False(d == (FMatrix4x4()));
        Assert::False(e == (FMatrix4x4()));
        Assert::False(f == (FMatrix4x4()));
        Assert::False(g == (FMatrix4x4()));
        Assert::False(h == (FMatrix4x4()));
        Assert::False(i == (FMatrix4x4()));
        Assert::False(j == (FMatrix4x4()));
        Assert::False(k == (FMatrix4x4()));
        Assert::False(l == (FMatrix4x4()));
        Assert::False(m == (FMatrix4x4()));
        Assert::False(n == (FMatrix4x4()));
        Assert::False(o == (FMatrix4x4()));
        Assert::False(p == (FMatrix4x4()));

        Assert::False(a.isIdentity());
        Assert::False(b.isIdentity());
        Assert::False(c.isIdentity());
        Assert::False(d.isIdentity());
        Assert::False(e.isIdentity());
        Assert::False(f.isIdentity());
        Assert::False(g.isIdentity());
        Assert::False(h.isIdentity());
        Assert::False(i.isIdentity());
        Assert::False(j.isIdentity());
        Assert::False(k.isIdentity());
        Assert::False(l.isIdentity());
        Assert::False(m.isIdentity());
        Assert::False(n.isIdentity());
        Assert::False(o.isIdentity());
        Assert::False(p.isIdentity());

        // Counterintuitive result - IEEE rules for NaN comparison are weird!
        Assert::False(a == (a));
        Assert::False(b == (b));
        Assert::False(c == (c));
        Assert::False(d == (d));
        Assert::False(e == (e));
        Assert::False(f == (f));
        Assert::False(g == (g));
        Assert::False(h == (h));
        Assert::False(i == (i));
        Assert::False(j == (j));
        Assert::False(k == (k));
        Assert::False(l == (l));
        Assert::False(m == (m));
        Assert::False(n == (n));
        Assert::False(o == (o));
        Assert::False(p == (p));
    });

    test("PerspectiveFarPlaneAtInfinityTest", [] ()
    {
        auto nearPlaneDistance = 0.125f;
        auto m = FMatrix4x4::perspective(1.0f, 1.0f, nearPlaneDistance, std::numeric_limits<float>::infinity());
        Assert::AreEqual(-1.0f, m.M33);
        Assert::AreEqual(-nearPlaneDistance, m.M43);
    });

    test("PerspectiveFieldOfViewFarPlaneAtInfinityTest", [] ()
    {
        auto nearPlaneDistance = 0.125f;
        auto m = FMatrix4x4::perspectiveFieldOfView(MathHelper::ToRadians(60.0f), 1.5f, nearPlaneDistance, std::numeric_limits<float>::infinity());
        Assert::AreEqual(-1.0f, m.M33);
        Assert::AreEqual(-nearPlaneDistance, m.M43);
    });

    test("PerspectiveOffCenterFarPlaneAtInfinityTest", [] ()
    {
        auto nearPlaneDistance = 0.125f;
        auto m = FMatrix4x4::perspectiveOffCenter(0.0f, 0.0f, 1.0f, 1.0f, nearPlaneDistance, std::numeric_limits<float>::infinity());
        Assert::AreEqual(-1.0f, m.M33);
        Assert::AreEqual(-nearPlaneDistance, m.M43);
    });

#pragma endregion

    return 0;
}