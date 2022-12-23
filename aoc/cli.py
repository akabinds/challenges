#!/usr/bin/env python

import os
from pathlib import Path
import typer
import subprocess
from dotenv import load_dotenv
from httpx_oauth.clients.github import GitHubOAuth2
from rich import print as rich_print
from functools import wraps
from asyncio import run
from datetime import date


def async_command(cli, *args, **kwargs):
    def decorator(async_func):
        @wraps(async_func)
        def sync_func(*_args, **_kwargs):
            return run(async_func(*_args, **_kwargs))

        cli.command(*args, **kwargs)(sync_func)

        return async_func

    return decorator


typer.Typer.async_command = async_command

load_dotenv()
cli = typer.Typer()


CLIENT_ID = os.environ["CLIENT_ID"]
CLIENT_SECRET = os.environ["CLIENT_SECRET"]
SESSION_ID = os.environ["SESSION_ID"]
USER_AGENT = os.environ["USER_AGENT"]


@cli.command()
def generate_generic(
    year: int = typer.Option(
        date.today().year,
        help="Specify the AOC year.",
        rich_help_panel="Optional Arguments",
    )
) -> None:
    """Generate a generic structure for AOC for the current year."""
    AOC_DIR = Path.home() / f"code/challenges/aoc/{year}"

    try:
        AOC_DIR.mkdir()
        (AOC_DIR / "inputs/").mkdir() 
    except* FileExistsError:
        rich_print(
            "[bold red]The specified target directory already exists![/bold red]"
        )
        raise typer.Exit()
    except* FileNotFoundError:
        rich_print("[bold red]The path specified does not exist![/bold red]")
        raise typer.Exit()

    rich_print(f"[green]Successfully generated generic AOC structure for year {year}![/green]")


@cli.command()
def generate_lang(
    year: int = typer.Option(
        date.today().year,
        help="Specify the AOC year.",
        rich_help_panel="Optional Arguments",
    ),
    lang: str = typer.Argument(...),
) -> None:
    """Generate a basic structure for AOC based on the language that the solutions will be completed in."""
    AOC_DIR = Path.home() / f"code/challenges/aoc/{year}"

    lang = lang.lower()
    lang_dir = AOC_DIR / f"{lang}"

    try:
        if lang == "rust":
            lang_dir.mkdir()
            os.chdir(lang_dir)
            subprocess.run(["cargo", "init"])
            (lang_dir / "src/bin/").mkdir()
        else:
            lang_dir.mkdir()
    except* FileExistsError:
        rich_print(
            "[bold red]The specified target directory already exists![/bold red]"
        )
        raise typer.Exit()
    except* FileNotFoundError:
        rich_print("[bold red]The path specified does not exist![/bold red]")
        raise typer.Exit()

    rich_print(f"[green]Successfully generated basic AOC structure for {lang.capitalize()} for the {year} AOC![/green]")


@cli.async_command()
async def input(
    year: int = typer.Option(
        date.today().year,
        help="Specify the AOC year.",
        rich_help_panel="Optional Arguments",
    ),
    day: int = typer.Option(
        date.today().day,
        help="Specify the day of the desired input.",
        rich_help_panel="Optional Arguments",
    ),
) -> None:
    """Fetch the input for the specified AOC year and problem day."""
    HOME = Path.home()
    INPUT = (
        HOME / f"code/challenges/aoc/{year}/inputs/day{day}.txt"
        if day >= 10
        else HOME / f"code/challenges/aoc/{year}/inputs/day0{day}.txt"
    )

    if INPUT.exists():
        rich_print(f"[bold red]The AOC input for year {year} and day {day} has already been fetched![/bold red]")
        raise typer.Exit()

    uri = f"https://adventofcode.com/{year}/day/{day}/input"
    auth_client = GitHubOAuth2(client_id=CLIENT_ID, client_secret=CLIENT_SECRET)

    async with auth_client.get_httpx_client() as client:
        res = await client.get(
            uri, cookies={"session": SESSION_ID}, headers={"User-Agent": USER_AGENT}
        )

        with open(INPUT, "w") as f:
            async for line in res.aiter_lines():
                f.write(line)

    rich_print(f"[green]Successfully fetched the AOC input for year {year} and day {day}![/green]")


if __name__ == "__main__":
    cli()
