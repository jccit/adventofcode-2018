import java.io.BufferedReader;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class Coord {
    public int x;
    public int y;

    Coord(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public String toString() {
        return "[" + x + ", " + y + "]";
    }
}

class Day6 {
    public static void main(String[] args) {
        Day6 app = new Day6();
        app.calcGrid();
        int largest = app.getLargestArea();
    }

    private ArrayList<Coord> coords;
    private int[][] grid;
    private int xMax = 0;
    private int yMax = 0;

    Day6() {
        coords = new ArrayList<Coord>();
        load("input.txt");

        grid = new int[xMax][yMax];
    }

    public void calcGrid() {
        for (int x = 0; x < xMax; x++) {
            for (int y = 0; y < yMax; y++) {
                // Find closest coord
                int i = 0;
                int closestIdx = 0;
                int closestDist = Integer.MAX_VALUE;
                for (Coord coord : coords) {
                    int dist = getDistance(x, y, coord);
                    if (dist < closestDist) {
                        closestDist = dist;
                        closestIdx = i;
                    }

                    i++;
                }

                grid[x][y] = closestIdx;
            }
        }
    }

    public void getLargestArea() {
        int[] sizes = new int[coords.size()];

        for (int x = 0; x < xMax; x++) {
            for (int y = 0; y < yMax; y++) {
                sizes[grid[x][y]]++;
            }
        }
    }

    private void load(String path) {
        try {
            File f = new File(path);
            BufferedReader buff = new BufferedReader(new FileReader(f));

            String line = "";
            Pattern regex = Pattern.compile("(\\d*), (\\d*)");

            while ((line = buff.readLine()) != null) {
                Matcher matcher = regex.matcher(line);
                matcher.find();

                int x = Integer.parseInt(matcher.group(1));
                int y = Integer.parseInt(matcher.group(2));

                if (x > xMax) xMax = x;
                if (y > yMax) yMax = y;

                Coord c = new Coord(x, y);
                coords.add(c);
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    private int getDistance(int x, int y, Coord coord) {
        int xDist = Math.abs(coord.x - x);
        int yDist = Math.abs(coord.y - y);

        return xDist + yDist;
    }
}
