#include "test_vim_math3d.h"

int main()
{
#pragma region DVector3Tests
    std::cout << "DVector3 Tests" << std::endl;

    test("DVector3CrossTest", []() {
        auto a = DVector3(1.0, 0.0, 0.0);
        auto b = DVector3(0.0, 1.0, 0.0);

        auto expected = DVector3(0.0, 0.0, 1.0);
        
        auto actual = a.cross(b);
        Assert::True(MathHelper::Equal(expected, actual), "Vector3f.Cross did not return the expected value.");
     });
#pragma endregion

    return 0;
}