+++
title = "Java A* Implementation"
date = "2014-04-24"

[taxonomies]
tags=["Programming"]
+++

There are a thousand implementations of A\* in Java. This one is mine.EDIT: If you're drawing your maps left to right, top to bottom, you're best off storing your maps as \[y]\[x] to avoid cache misses and to improve data locality. The code below assumes \[x]\[y] because I wrote it for someone else.`import java.awt.Point;
import java.util.ArrayList;

public class AStar {
public static Point[] findPath(double[][] map, Point start, Point finish) {
// These are for running A*.
Point current = null;
ArrayList <point> candidates = new ArrayList<point>();
Point[][] parent = new Point[map.length]map[0].length];
double[][] cost = new double[map.length]map[0].length]; // All costs start at infinity, but Java inits them to zero.
double[][] costToGoal = new double[map.length]map[0].length];
// Set initial costs to infinity.
for(int i=0; i < map.length; i++) {
for(int j=0; j < map[i].length; j++) {
cost[i][j] = Float.POSITIVE*INFINITY;
costToGoal[i][j] = Float.POSITIVE_INFINITY;
}
}
// We make start its own parent so that we know it's visited.  
 // This may have problems when reversing, but we can safely do so because we have the start point as an argument.
parent[start.x][start.y] = start;
cost[start.x][start.y] = 0;
costToGoal[start.x][start.y] = heuristic(map, start.x, start.y, finish.x, finish.y);
candidates.add(start);
while(!candidates.isEmpty()) {
// Do an O(n) search for the smallest candidate. In a densely connected graph, a minheap may drive the cost to O(n^2 log n).
// Since this isn't really densely connected, we could switch for a performance boost.
Point minCostCandidate = null;
double minCost = Float.POSITIVE_INFINITY;
for(Point p : candidates) {
if(cost[p.x][p.y] + costToGoal[p.x][p.y] < minCost) {
minCostCandidate = p;
minCost = cost[p.x][p.y] + costToGoal[p.x][p.y];
}
}
candidates.remove(minCostCandidate);
current = minCostCandidate;
if(current.equals(finish)) {
break;
}
// We've got the shortest path so far. Now let's figure out the cost of the eight items around this candidate.
// If a neighbor has a \_higher* cost, make this node the parent and decrease the cost.
for(int dy=-1; dy<2; dy++) {
for(int dx=-1; dx<2; dx++) {
// Short-hand for our neighbor's location.
int nbrx = current.x+dx;
int nbry = current.y+dy;
// Make sure it's a valid spot on the map.
if(dx==0 && dy==0) { continue; } // Skip the current point. Can't be your own parent.
if(!isInsideMap(map, nbrx, nbry)) { continue; } // Don't plot outside the map.
// Distance from the start to here plus the distance from here to the neighbor.
double distToNeighbor = cost[current.x][current.y] + distanceSquared(map, current.x, current.y, nbrx, nbry);
// If we have a faster way to get here, update it!
if(distToNeighbor < cost[nbrx][nbry]) {
cost[nbrx][nbry] = distToNeighbor;
costToGoal[nbrx][nbry] = heuristic(map, nbrx, nbry, finish.x, finish.y);
parent[nbrx][nbry] = current;
candidates.add(new Point(nbrx, nbry));
}
}
}
}
if(!current.equals(finish)) {
return null; // No path to goal.
}
ArrayList <point> prepath = new ArrayList<point>();
while(!current.equals(start)) {
prepath.add(current);
current = parent[current.x][current.y];
}
Point[] path = new Point[prepath.size()]; // Reverse the path before returning it.
for(int i=0; i < path.length; i++) {
path[i] = prepath.get(prepath.size()-1-i);
}
return path;
}
public static boolean isInsideMap(double[][] map, int x, int y) {
return x >= 0 && x < map.length && y >= 0 && y < map[0].length;
}
public static double heuristic(double[][] map, int ax, int ay, int goalx, int goaly) {
double dx = ax - goalx;
double dy = ay - goaly;
double dz = map[ax][ay] - map[goalx][goaly];
return dx*dx + dy*dy + dz*dz;
}
public static double distanceSquared(double[][] map, int ax, int ay, int bx, int by) {
final double UPHILL_COST_SCALAR = 1.0; // Going up hill costs 3x more than going down hill.
final double DOWNHILL_COST_SCALAR = 1.0; // Going down hill costs just as much as going flat.
double deltaDistance = (ax-bx)_(ax-bx) + (ay-by)_(ay-by);
double deltaHeight = map[ax][ay] - map[bx][by];
if(deltaHeight < 0) {
deltaHeight = Math.max(0, deltaHeight*DOWNHILL_COST_SCALAR); // We are assuming that moving downhill has a non-negative cost.
} else if(deltaHeight > 0) {
deltaHeight *= UPHILL_COST_SCALAR;
} else {
// No change.
}
return deltaDistance + deltaHeight\*deltaHeight;
}
}

</point></point></point></point>`And some helpful main to produce pretty outputs:`	public static void main(String[] args) {
		final int XSIZE = 40;
		final int YSIZE = 40;
		
		// Run this to test.
		double[][] map = new double[XSIZE][YSIZE];
		Random rand = new Random();
		
		for(int i=0; i < XSIZE; i++) {
			for(int j=0; j < YSIZE; j++) {
				map[i][j] = rand.nextInt(2);
			}
		}
		Point start = new Point(rand.nextInt(XSIZE), rand.nextInt(YSIZE));
		Point end = new Point(rand.nextInt(XSIZE), rand.nextInt(YSIZE));
		
		Point[] path = findPath(map, start, end);
		
		// Draw the map
		char[][] output = new char[XSIZE][YSIZE];
		for(int i=0; i < XSIZE; i++) {
			for(int j=0; j < YSIZE; j++) {
				if(map[i][j] != 0) {
					output[i][j] = '#';
				} else {
					output[i][j] = ' ';
				}
			}
		}
		
		// Draw the path on the map.
		if(path != null) {
			for(Point p : path) {
				output[p.x][p.y] = '.';
			}
		}
		
		// Draw the start and end.
		output[start.x][start.y] = 'S';
		output[end.x][end.y] = 'E';
		
		// Print everything.
		for(int j=0; j < YSIZE; j++) {
			for(int i=0; i < XSIZE; i++) {
				System.out.print(output[i][j]);
			}
			System.out.println();
		}
	}
`And some selected sample output.

```
     # #  #### # ### # #     ##   #   #
   ### #E.### ##  ##  ##     #  #######
 # # # # #.... ## #   # ##  ## #   # #
##### #  # ###.##    # ## ## ### # ##  #
## ## #### ## #.# # ## ## #  # ######  #
##   ### # ## ##.# # ## ###    #   #  #
#### #  ### #   #.# ###### ## # ###### #
 #  ##    ##  ### .########## #   ## #
    ## # #  ##     . # ##### # # # #####
 ###   #  ###   ##  .   #     ##    ####
#      ##  #  #### # .##  ### ##  #####
##   ##    # #  # ####S # # #    # ### #
```



```
#   #  #  #   # #  ## # ## # #######  ##
# ##### #  #   #   #  #. ##    ###..# #
## ##  ## ##........... .......... #E# #
 ######   #.  ##   # ## #  ##    ## # ##
 #   ## ##.## ##   # # #  ###  ####   ##
## #    #. #      #  ## # #      ###  #
 #### ##.###  #####    ### # #### # ###
#  # ##.   # ####  #   ###  # #   ##
 #### S# #    ####     #  ### #  ##  ###
  ##  #  ##  # #   #  ## #  ##### ## ###
#  ## ## #  # #     # ##    ## ### # ##
```
