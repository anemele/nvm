"""Nodejs Version Manager"""

import click

from .cmd import cmd_list


class OrderedGroup(click.Group):
    def list_commands(self, _ctx):
        return self.commands.keys()


cli = OrderedGroup(help=__doc__)


@cli.command(name="ls")
def cli_list():
    """list installed versions"""
    cmd_list()


@cli.command(name="lr")
def cli_list_remote():
    """list remote versions"""
    click.echo("List of remote commands")


@cli.command(name="use")
@click.argument("version", type=str)
def cli_use(version: str):
    """use a specific version"""
    click.echo(f"Using version {version}")


@cli.command(name="add")
@click.argument("version", type=str)
def cli_install(version: str):
    """install a specific version"""
    click.echo(f"Installing version {version}")


@cli.command(name="rm")
@click.argument("version", type=str)
def cli_remove(version: str):
    """remove a specific version"""
    click.echo(f"Removing version {version}")


@cli.command(name="clean")
def cli_clean():
    """clean up unused versions"""
    click.echo("Cleaning up unused versions")


def main():
    try:
        cli()
    except Exception as e:
        click.echo(f"Error: {e}")


if __name__ == "__main__":
    main()
