/* Copyright (C) 2020 Jeremiah Orians, 2025 Julie Bettens
 * This file is part of zkbootstrap.
 *
 * stage0 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * stage0 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with stage0.  If not, see <http://www.gnu.org/licenses/>.
 */

// Based on stage0/stage0/High_level_prototypes/stage0_monitor.c

void line_Comment()
{
	int c = getchar();
	// NOTE: assumes file ends with a line terminator
	while((10 != c) && (13 != c))
	{
		c = getchar();
	}
}

int hex(int c)
{
	/* Clear out line comments */
	if((';' == c) || ('#' == c))
	{
		line_Comment();
		return -1;
	}

	/* Deal with non-hex chars*/
	if('0' > c) return -1;

	/* Deal with 0-9 */
	if('9' >= c) return (c - 48);

	/* Convert a-f to A-F*/
	c = c & 0xDF;

	/* Get rid of everything below A */
	if('A' > c) return -1;

	/* Deal with A-F */
	if('F' >= c) return (c - 55);

	/* Everything else is garbage */
	return -1;
}

/* Standard C main program */
	int toggle = 0;
	int holder = 0;

	int R0;
	int c;
int main()
{
	j_prepare();

	for(c = getchar(); EOF != c; c = getchar())
	{
		R0 = hex(c);
		if(0 <= R0)
		{
			if(toggle)
			{
				putchar((holder * 16) + R0);
				holder = 0;
			}
			else
			{
				holder = R0;
			}

			toggle = !toggle;
		}
	}

	j_finalize_and_halt();
}
