package dicey_temperatures_java;

import java.util.Scanner;

public class App {
  public static void main(String[] args) {
    System.out.println("Guess the sum of two dice!");

    System.out.println("Please input your guess (between 2 and 12): ");

    Scanner scanner = new Scanner(System.in);
    String guess = scanner.nextLine();

    System.out.println("You guessed: " + guess);
  }
}
