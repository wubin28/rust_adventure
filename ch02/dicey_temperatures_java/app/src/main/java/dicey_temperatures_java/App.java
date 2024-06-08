package dicey_temperatures_java;

import java.util.Optional;
import java.util.Random;
import java.util.Scanner;

public class App {
  public static void main(String[] args) {
    System.out.println("Guess the sum of two dice!");

    Random random = new Random();
    int sumOfTwoDice = random.nextInt(11) + 2; // 生成 2 到 12 之间的随机数

    System.out.println("The sum of two dice is: " + sumOfTwoDice);

    Optional<Integer> previousGuess = Optional.empty();

    Scanner scanner = new Scanner(System.in);

    while (true) {
      System.out.println("Please input your guess (between 2 and 12): ");

      String guess = scanner.nextLine();

      int guessInt;
      try {
        guessInt = Integer.parseInt(guess.trim());
      } catch (NumberFormatException e) {
        System.out.println("Please type a number!");
        continue;
      }

      if (guessInt < 2 || guessInt > 12) {
        System.out.println("Please type a number between 2 and 12!");
        continue;
      }

      System.out.println("You guessed: " + guessInt);

      if (previousGuess.isEmpty()) {
        if (guessInt < sumOfTwoDice || guessInt > sumOfTwoDice) {
          System.out.println("You guessed it wrong on the first try");
        } else {
          System.out.println("You win!");
          break;
        }
        previousGuess = Optional.of(guessInt);
      } else {
        int prev = previousGuess.get();
        int previousDiff = Math.abs(prev - sumOfTwoDice);
        int currentDiff = Math.abs(guessInt - sumOfTwoDice);

        if (guessInt == sumOfTwoDice) {
          System.out.println("You win!");
          break;
        } else if (currentDiff < previousDiff) {
          System.out.println("Hotter.");
        } else if (currentDiff > previousDiff) {
          System.out.println("Colder.");
        } else {
          System.out.println("Neither colder nor hotter.");
        }
        previousGuess = Optional.of(guessInt);
      }
    }
  }
}
