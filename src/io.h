#ifndef IO_H
#define IO_H

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

struct cColor{
	short int r;
	short int g;
	short int b;
};

namespace io{

	void Init();
	void Cleanup();

	void drawChar(char c, int x, int y, cColor cC);
	
	void Clear();
	void Flip();
	
}

#endif
