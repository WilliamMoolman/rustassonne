import { useState, useEffect, useRef, useCallback } from "react";
import "./App.css";
import init, { Game } from "./pkg/rustassonne.js";

function getTilename(tileId) {
  return `tile-${String.fromCharCode(97 + tileId)}.png`;
}

function Canvas({ width, height, tiles, tilesRotation, handleTileClick }) {
  return (
    <div className="Canvas">
      <div
        className="Canvas-grid"
        style={{ gridTemplateColumns: `repeat(${width}, 80px)` }}
      >
        {tiles.map((tileId, i) => {
          if (tileId === 255) {
            return <div className="empty-tile" />;
          }
          if (tileId === 254) {
            return (
              <div
                className="clickable-tile"
                onClick={() => handleTileClick(i, 0)}
              />
            );
          }
          return (
            <img
              src={`./img/${getTilename(tileId)}`}
              alt={getTilename(tileId)}
              className={`rotate${90 * tilesRotation[i]} tile`}
            />
          );
        })}
      </div>
    </div>
  );
}

function Sidebar({ nextTile }) {
  return (
    <div className="Sidebar">
      <h3>Next Tile</h3>
      <img
        src={`./img/tile-${String.fromCharCode(97 + nextTile)}.png`}
        alt={`tile-${String.fromCharCode(97 + nextTile)}.png`}
        className={`nexttile`}
      />
    </div>
  );
}

function Board({
  nextTile,
  width,
  height,
  tiles,
  tilesRotation,
  handleTileClick,
}) {
  return (
    <div className="Board">
      <Sidebar nextTile={nextTile} />
      <Canvas
        width={width}
        tiles={tiles}
        tilesRotation={tilesRotation}
        handleTileClick={handleTileClick}
      />
    </div>
  );
}

function Remaining({ remaining }) {
  return (
    <div className="Remaining">
      <h2> Remaining </h2>
      <div className="Remaining-bar">
        {remaining.map((object, i) => (
          <div className="Remaining-cardtype">
            <p> {object}x </p>
            <img
              src={`./img/tile-${String.fromCharCode(97 + i)}.png`}
              alt={`tile-${String.fromCharCode(97 + i)}.png`}
              className={"remaining-tile"}
            />
          </div>
        ))}
      </div>
    </div>
  );
}

const useGame = () => {
  const gameRef = useRef(null);
  const [remaining, setRemaining] = useState([]);
  const [boardWidth, setBoardWidth] = useState(0);
  const [boardHeight, setBoardHeight] = useState(0); // Placeholder for height state
  const [boardTiles, setBoardTiles] = useState([]);
  const [nextTile, setNextTile] = useState(0);
  const [boardTilesRotation, setBoardTilesRotation] = useState([]);
  const [tilePlaced, setTilePlaced] = useState(false);
  const [tilePlacement, setTilePlacement] = useState([]);

  const initializeGame = useCallback(async () => {
    await init().then(() => {
      gameRef.current = Game.standard();
      console.log("Game initialized:", gameRef.current);
    });
  }, []);

  const refresh = useCallback(() => {
    if (gameRef.current) {
      const remaining_tiles = Array.from(gameRef.current.get_remaining());
      const width = gameRef.current.width();
      const tiles = Array.from(gameRef.current.tiles());
      const tilesRotation = Array.from(gameRef.current.tiles_rotation());
      const gameNextTile = gameRef.current.next_tile();

      setRemaining((prev) =>
        JSON.stringify(prev) !== JSON.stringify(remaining_tiles)
          ? remaining_tiles
          : prev,
      );
      setBoardWidth((prev) => (prev !== width ? width : prev));
      setBoardTiles((prev) =>
        JSON.stringify(prev) !== JSON.stringify(tiles) ? tiles : prev,
      );
      setBoardTilesRotation((prev) =>
        JSON.stringify(prev) !== JSON.stringify(tilesRotation)
          ? tilesRotation
          : prev,
      );
      setNextTile((prev) => (prev !== gameNextTile ? gameNextTile : prev));

      console.log("Refreshed!");
      console.log("Next Tile", getTilename(gameNextTile));
    }
  }, []);

  const placeTile = useCallback(() => {
    if (tilePlaced && gameRef.current) {
      const [positionId, rotation] = tilePlacement;
      gameRef.current.place_next(positionId, rotation);
      console.log("Placed Tile");
      setTilePlaced(false);
      refresh();
    }
  }, [tilePlaced, tilePlacement, refresh]);

  useEffect(() => {
    const initializeAndRefresh = async () => {
      await initializeGame();
      refresh();
    };
    initializeAndRefresh();
  }, [initializeGame, refresh]);

  useEffect(() => {
    placeTile();
  }, [tilePlaced, placeTile]);

  return {
    remaining,
    boardWidth,
    boardHeight,
    boardTiles,
    nextTile,
    boardTilesRotation,
    setTilePlacement,
    setTilePlaced,
  };
};

const App = () => {
  const {
    remaining,
    boardWidth,
    boardHeight,
    boardTiles,
    nextTile,
    boardTilesRotation,
    setTilePlacement,
    setTilePlaced,
  } = useGame();

  function handleClick(positionId, rotation) {
    setTilePlacement([positionId, rotation]);
    setTilePlaced(true);
  }

  return (
    <div className="App">
      <Board
        nextTile={nextTile}
        width={boardWidth}
        tiles={boardTiles}
        tilesRotation={boardTilesRotation}
        handleTileClick={handleClick}
      />
      <Remaining remaining={remaining} />
    </div>
  );
};

export default App;
