SRC_DIR = src
CC=g++
CFLAGS= -Wall -g -std=c++17 $(shell sdl2-config --cflags)
LDFLAGS= $(shell sdl2-config --libs) -lSDL2_image
SOURCES =$(wildcard $(SRC_DIR)/*.cpp)
TARGET = loekchipz
OBJS = $(SOURCES:.cpp=.o)
.PHONY:$(TARGET)
$(TARGET): 
	$(CC) $(CFLAGS) -o $(TARGET) $(SOURCES) $(LDFLAGS) 

clean:
	$(RM) $(OBJS)
