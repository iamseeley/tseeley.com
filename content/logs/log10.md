---
title: log10 
date: 2024-08-22
---

**felt very focused today!**

got caught up with my sicp readings, so i watched the next lecture in brian harvey's cs61a class.

i started vannevar bush's [*As We May Think*](https://www.w3.org/History/1945/vbush/vbush.shtml) essay.

then took a small detour and worked on a maze generation playground site that was inspired by andrew healey's recent blog post [*Generating Mazes*](https://healeycodes.com/generating-mazes).

it turns out mazes are very cool. 

it's especially fun to watch a maze be generated in real-time.

i'm going to try to implement - on my own - a few different maze generation algorithms and add them to my maze gen playground. i'll start with some of the algos andrew used in his experiment.

so far i've added a random depth-first search algorithm

```typescript
export class DFS {
  *generateSteps(maze: MazeInterface, startCell: CellInterface): Generator<{ currentCell: CellInterface }> {
    const stack: CellInterface[] = [startCell];
    startCell.visited = true;

    while (stack.length > 0) {
        const currentCell = stack[stack.length - 1]; 
        yield { currentCell };

        const unvisitedNeighbors = maze.getNeighbors(currentCell).filter(neighbor => !neighbor.visited);
      
        if (unvisitedNeighbors.length > 0) {
        
        const randomIndex = Math.floor(Math.random() * unvisitedNeighbors.length);
        const chosenNeighbor = unvisitedNeighbors[randomIndex];
        
        currentCell.removeWall(chosenNeighbor);
        chosenNeighbor.visited = true;
        
        stack.push(chosenNeighbor);
      } else {
        stack.pop();
      }
    }
  }
}
```

basically, it starts with a grid of cells with all the walls intact. it then begins at a random cell and marks it visited. 

if the current cell has unvisited neighbors it chooses one randomly, and then removes the walls beteen them and moves to that neighbor. 

otherwise it backtracks to the previous cell.

it continues until all cells are visited.

here's the site: [maze gen playground](https://maze-playground.deno.dev/)

i plan on making the visualization nicer (highlighting the current cell with a color you can actually see and reducing the thickness of the walls) 

going to add one algo at a time!

~

after that side quest i got back to work on my web analytics tracker.

i improved the design of some of the charts (i'm using recharts), but they are still wonky. 

handling the sizing of the chart and placing labels / values inside the chart where i want them has been painful. 

and that's that for today.

    


