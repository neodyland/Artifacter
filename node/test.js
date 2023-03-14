const artifacter = require(".");
const UID = 827106332;
const fs = require("fs");

function prompt(question, defaultAnswer) {
    const readline = require("readline").createInterface({
        input: process.stdin,
        output: process.stdout
    });
    return new Promise(resolve => {
        readline.question(question, answer => {
            readline.close();
            resolve(answer || defaultAnswer);
        });
    });
}

((async() => {
    await artifacter.load();
    const characters = await artifacter.getCharacters(UID);
    console.log(characters);
    const character = Number(await prompt("Enter a character name: ", characters[0]));
    const now = Date.now();
    const data = await artifacter.generate(UID, character,"ja","png","Normal");
    console.log("Time taken: " + (Date.now() - now) + "ms");
    fs.writeFileSync("test.png", data);
    console.log("Done");
}))()