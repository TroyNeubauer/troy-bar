
CPPFLAGS = -g
LDFLAGS = -g

SRCS = Main.cpp
OBJS = $(subst .cc,.o,$(SRCS))

troybar: $(OBJS)
	g++ $(LDFLAGS) -o troybar $(OBJS) $(LDLIBS)

main.o: Main.cpp
	g++ $(CPPFLAGS) Main.cpp
