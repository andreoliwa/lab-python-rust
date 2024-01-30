import click
from slugify import slugify
from lab_python_rust import _lab_python_rust as rust


@click.group()
def my_cli():
    """Python CLI entry point."""


@my_cli.command()
@click.argument("pieces", nargs=-1)
def slug(pieces: tuple[str, ...]):
    joined = " ".join(pieces)
    print(f"Pieces: {joined} from Python ({slugify(joined)})")
    print("sum_as_string:", rust.sum_as_string(5, 20))
    print(rust.hello("John Doe"))
