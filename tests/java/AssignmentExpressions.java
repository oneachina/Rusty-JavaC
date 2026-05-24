public class AssignmentExpressions {
    static int left = 10;
    static int right = 20;
    static int[] values = new int[]{3, 5, 7};

    public static void main(String[] args) {
        int x = 1;
        int y = 2;
        int picked = left > right ? (x = 11) : (y = 22);

        System.out.println(picked);
        System.out.println(x + "," + y);
        System.out.println(values[1] += values[0] -= 2);
        System.out.println(values[0] + "," + values[1] + "," + values[2]);
    }
}
