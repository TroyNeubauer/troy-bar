
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include <X11/Xatom.h>
#include <X11/Xmu/CurUtil.h>
#include <X11/Xcursor/Xcursor.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
//#include "X11/bitmaps/gray"


int main(int argc, char *argv[]) 
{
	const char *display_name = nullptr;

	char* program_name=argv[0];

	Display* dpy = XOpenDisplay(display_name);
	if (!dpy)
	{
		fprintf(stderr, "%s:  unable to open display '%s'\n", program_name, XDisplayName (display_name));
		exit(2);
	}
	int screen = DefaultScreen(dpy);
	Window root = RootWindow(dpy, screen);

	XStoreName(dpy, root, "adads");  
  
	XCloseDisplay(dpy);
	exit (0);
}



