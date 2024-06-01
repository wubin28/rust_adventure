package dicey_temperatures_java;

import java.util.Random;
import java.util.Scanner;

public class App {
  public static void main(String[] args) {
    System.out.println("Guess the sum of two dice!");

    Random random = new Random();
    int sumOfTwoDice = random.nextInt(11) + 2; // 生成 2 到 12 之间的随机数

    System.out.println("The sum of two dice is: " + sumOfTwoDice);

    System.out.println("Please input your guess (between 2 and 12): ");

    Scanner scanner = new Scanner(System.in);
    String guess = scanner.nextLine();

    System.out.println("You guessed: " + guess);
  }
}
