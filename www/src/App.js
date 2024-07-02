import { useState, useEffect, useRef } from 'react';
import './App.css';
import init, { Game } from './pkg/rustassonne.js';

function Canvas({ width, height, tiles, tilesRotation, handleTileClick }) {
    return (
        <div className="Canvas">
            <div className="Canvas-grid" style={{ gridTemplateColumns: `repeat(${width}, 78px)` }}>
                {tiles.map((tileId, i) => {
                    if (tileId === 255) {
                        return <div className="empty-tile" />
                    }
                    if (tileId === 254) {
                        return <div className="clickable-tile" onClick={() => handleTileClick(i, 0)} />
                    }
                    return (<img
                        src={`./img/tile-${String.fromCharCode(97 + tileId)}.png`}
                        alt={`tile-${String.fromCharCode(97 + tileId)}.png`}
                        className={`rotate${90 * tilesRotation[i]} tile`}
                    />)
                })}
            </div>
        </div>
    )
}

function Sidebar({nextTile}) {
    return (
        <div className="Sidebar">
            <h3>Next Tile</h3>
            <img
                src={`./img/tile-${String.fromCharCode(97 + nextTile)}.png`}
                alt={`tile-${String.fromCharCode(97 + nextTile)}.png`}
                className={`nexttile`}
            />
        </div>
    )
}

function Board({ nextTile, width, height, tiles, tilesRotation, handleTileClick }) {
    return (
        <div className="Board">
            <Sidebar nextTile={nextTile}/>
            <Canvas width={width}
                tiles={tiles}
                tilesRotation={tilesRotation}
                handleTileClick={handleTileClick} />
        </div>
    )
}

function Remaining({ remaining }) {
    return (<div className="Remaining">
        <h2> Remaining </h2>
        <div className="Remaining-bar">
            {remaining.map((object, i) => <div className="Remaining-cardtype"><p> {object}x </p><img
                src={`./img/tile-${String.fromCharCode(97 + i)}.png`}
                alt={`tile-${String.fromCharCode(97 + i)}.png`}
            /></div>)}
        </div>
    </div>)
}

function App() {
    const gameRef = useRef(null);
    const [initialized, setInitialized] = useState(false);
    const [remaining, setRemaining] = useState([]);
    const [boardWidth, setBoardWidth] = useState(0);
    const [boardHeight, setBoardHeight] = useState(0);
    const [boardTiles, setBoardTiles] = useState([]);
    const [nextTile, setNextTile] = useState(0);
    const [boardTilesRotation, setBoardTilesRotation] = useState([]);
    const [tilePlaced, setTilePlaced] = useState(false);
    const [tilePlacement, setTilePlacement] = useState([]);

    useEffect(() => {
        async function refresh() {
            const remaining_tiles = Array.from(gameRef.current.get_remaining());
            setRemaining(remaining_tiles);
            const width = gameRef.current.width();
            setBoardWidth(width);
            const tiles = Array.from(gameRef.current.tiles());
            setBoardTiles(tiles);
            const tilesRotation = Array.from(gameRef.current.tiles_rotation());
            setBoardTilesRotation(tilesRotation);
            const gameNextTile = gameRef.current.next_tile();
            setNextTile(gameNextTile);
            console.log("Refreshed!", gameNextTile);
        }
        if (initialized) {
            refresh();
        }
        if (tilePlaced) {
            const positionId = tilePlacement[0];
            const rotation = tilePlacement[1];
            gameRef.current.place_next(positionId, rotation);
            setTilePlaced(false);
        }
    }, [initialized, tilePlaced, tilePlacement]);

    function handleClick(positionId, rotation) {
        setTilePlacement([positionId, rotation]);
        setTilePlaced(true);
    } 

    useEffect(() => {
        const initializeGame = async () => {
            await init().then(() => { // Initialize the wasm module
                gameRef.current = Game.standard(); // Create and store the Game object
                console.log('Game initialized:', gameRef.current);
                // const remaining_tiles = Array.from(gameRef.current.get_remaining());
                // setRemaining(remaining_tiles);
                // const width = gameRef.current.width();
                // setBoardWidth(width);
                // const tiles = Array.from(gameRef.current.tiles());
                // setBoardTiles(tiles);
                // const tilesRotation = Array.from(gameRef.current.tiles_rotation());
                // setBoardTilesRotation(tilesRotation);
                // setNextTile(gameRef.current.next_tile());
                // refresh();
            });
            setInitialized(true);
        };

        initializeGame();
    }, []);

    if (!initialized || remaining.length === 0) {
        return <div>Loading...</div>; // Show loading state while initializing
    }

    return (
        <div className="App">
            <Board 
                nextTile={nextTile}
                width={boardWidth}
                tiles={boardTiles}
                tilesRotation={boardTilesRotation} 
                handleTileClick={handleClick} />
            <Remaining remaining={remaining} />
        </div>
    );
}

export default App;
