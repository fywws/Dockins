#include <iostream>
#include <cmath>
#include "MathLibrary.h" // Replace with the actual path to your MathLibrary header

using namespace std;

class MathLibrary
{
public:
    static double add(double a, double b)
    {
        return a + b;
    }

    static double subtract(double a, double b)
    {
        return a - b;
    }

    static double multiply(double a, double b)
    {
        return a * b;
    }

    static double divide(double a, double b)
    {
        if (b != 0)
        {
            return a / b;
        }
        else
        {
            std::cerr << "Error: Division by zero!" << std::endl;
            return NAN; // Not-A-Number
        }
    }

    static double power(double base, double exponent)
    {
        return std::pow(base, exponent);
    }

    static double squareRoot(double x)
    {
        if (x >= 0)
        {
            return std::sqrt(x);
        }
        else
        {
            std::cerr << "Error: Cannot calculate square root of a negative number!" << std::endl;
            return NAN;
        }
    }
    static double add(double a, double b)
    {
        return a + b;
    }

    static double subtract(double a, double b)
    {
        return a - b;
    }

    static double multiply(double a, double b)
    {
        return a * b;
    }

    static double divide(double a, double b)
    {
        if (b != 0)
        {
            return a / b;
        }
        else
        {
            std::cerr << "Error: Division by zero!" << std::endl;
            return NAN; // Not-A-Number
        }
    }

    static double power(double base, double exponent)
    {
        return std::pow(base, exponent);
    }

    static double squareRoot(double x)
    {
        if (x >= 0)
        {
            return std::sqrt(x);
        }
        else
        {
            std::cerr << "Error: Cannot calculate square root of a negative number!" << std::endl;
            return NAN;
        }
    }
};

int main()
{
    // Declare variables
    double num1 = 10.5, num2 = 3.2, result;
    int exp = 4;

    cout << "Performing math calculations:\n";

    // Addition
    result = MathLibrary::add(num1, num2);
    cout << "\nAddition: " << num1 << " + " << num2 << " = " << result << endl;

    // Subtraction
    result = MathLibrary::subtract(num1, num2);
    cout << "\nSubtraction: " << num1 << " - " << num2 << " = " << result << endl;

    // Multiplication
    result = MathLibrary::multiply(num1, num2);
    cout << "\nMultiplication: " << num1 << " * " << num2 << " = " << result << endl;

    try
    {
        // Division
        result = MathLibrary::divide(num1, num2);
        cout << "\nDivision: " << num1 << " / " << num2 << " = " << result << endl;
    }
    catch (const char *err_msg)
    {
        cout << "\nERROR: " << err_msg << endl;
    }

    // Power calculation
    result = MathLibrary::power(num1, exp);
    cout << "\nPower: " << num1 << "^" << exp << " = " << result << endl;

    // Square Root
    result = MathLibrary::squareRoot(num1);
    cout << "\nSquare Root: sqrt(" << num1 << ") = " << result << endl;

    return 0;
}
