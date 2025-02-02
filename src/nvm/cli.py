"""Nodejs Version Manager"""

import click


class OrderedGroup(click.Group):
    def list_commands(self, _ctx):
        return self.commands.keys()


cli = OrderedGroup(help=__doc__)


@cli.command(name="ls")
def cmd_list():
    """list installed versions"""
    click.echo("List of commands")


@cli.command(name="lr")
def cmd_list_remote():
    """list remote versions"""
    click.echo("List of remote commands")


@cli.command(name="use")
@click.argument("version", type=str)
def cmd_use(version: str):
    """use a specific version"""
    click.echo(f"Using version {version}")


@cli.command(name="add")
@click.argument("version", type=str)
def cmd_install(version: str):
    """install a specific version"""
    click.echo(f"Installing version {version}")


@cli.command(name="rm")
@click.argument("version", type=str)
def cmd_remove(version: str):
    """remove a specific version"""
    click.echo(f"Removing version {version}")


@cli.command(name="clean")
def cmd_clean():
    """clean up unused versions"""
    click.echo("Cleaning up unused versions")


def main():
    try:
        cli()
    except Exception as e:
        click.echo(f"Error: {e}")


if __name__ == "__main__":
    main()
