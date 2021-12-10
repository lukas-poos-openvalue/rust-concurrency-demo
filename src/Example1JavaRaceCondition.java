import java.util.ArrayList;
import java.util.List;

public class Example1JavaRaceCondition {

    private static class Counter {
        public int value = 0;
    }

    public static void main(String... args) throws Exception {
        final var counter = new Counter();
        final List<Thread> threads = new ArrayList<>();

        for (var i = 0; i < 10; i++) {
            final var thread = new Thread(() -> {
                for (var j = 0; j < 10000; j++) {
                    counter.value += 1;
                }
            });
            thread.start();
            threads.add(thread);
        }

        for (var thread : threads) {
            thread.join();
        }

        System.out.println("Value of counter.value: " + counter.value);
    }
}