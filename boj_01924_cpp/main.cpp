#include <iostream>

int main() {
    int monthDay[12] = {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    int month;       //월을 입력받음
    int day;         //일을 입력받음
    int daySum = 0;  //입력한 날짜가 첫날에서 며칠 지났는지를 출력
    std::cin >> month >> day;

    for (int i = 0; i < month - 1; i++) {
        daySum += monthDay[i];
    }

    daySum += day;

    switch (daySum % 7) {
        case 1:
            std::cout << "MON";
            break;
        case 2:
            std::cout << "TUE";
            break;
        case 3:
            std::cout << "WED";
            break;
        case 4:
            std::cout << "THU";
            break;
        case 5:
            std::cout << "FRI";
            break;
        case 6:
            std::cout << "SAT";
            break;
        case 0:
            std::cout << "SUN";
            break;
    }
}