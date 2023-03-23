import asyncio
import time

import artifacter_py


UID = 827106332


async def main():
    await artifacter_py.load()
    charactes = await artifacter_py.get_characters(UID)
    print(charactes)
    character = int(input("Enter a character id: "))
    now = time.time()
    bytes = await artifacter_py.generate(UID,character,"ja","png","Normal")
    print("Time taken: ",time.time() - now)
    with open("test.png","wb") as f:
        f.write(bytes)
    print("Done")

asyncio.run(main())
