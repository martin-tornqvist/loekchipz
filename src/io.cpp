#include "io.h"
#include <iostream>

namespace g {

const int SCREEN_WIDTH = 1280;
const int SCREEN_HEIGHT = 720;
const int FONT_W = 16;
const int FONT_H = 16;
}

namespace{
	SDL_Window* _window;
	SDL_Renderer* _renderer;
	SDL_Texture* _font;
}

namespace io {

void Init(){

	SDL_Init(SDL_INIT_VIDEO);
    
	_window = SDL_CreateWindow("LoekChipz",
				   SDL_WINDOWPOS_UNDEFINED,
				   SDL_WINDOWPOS_UNDEFINED,
				   g::SCREEN_WIDTH,
				   g::SCREEN_HEIGHT,
				   0);
    
	_renderer = SDL_CreateRenderer(_window, -1, SDL_RENDERER_ACCELERATED);
   
    
	SDL_Surface* loadedSurface = IMG_Load("gfx/16x16.png");
    
	if(loadedSurface == NULL)
		std::cout << "Could not open " << "16x16.png" << std::endl;
    
	SDL_SetColorKey(loadedSurface,SDL_TRUE, SDL_MapRGB(loadedSurface->format, 255, 255, 255 ) );
    
	_font = SDL_CreateTextureFromSurface(_renderer, loadedSurface);
    
	SDL_FreeSurface(loadedSurface);
	
  
	
} // Init

void Cleanup(){
	SDL_DestroyTexture(_font);
	SDL_DestroyRenderer(_renderer);
	SDL_DestroyWindow(_window);
	SDL_Quit();
} // Cleanup

void drawChar(char c, int x, int y, cColor cC){
	SDL_Rect srcRct;
	if(c >= ' ' && c <= '/'){
		srcRct.y = 0;
		srcRct.x = ((int)c-32)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;
	}
	else if(c >= '0' && c <= '?'){
		srcRct.y = 1*g::FONT_H;
		srcRct.x = ((int)c-48)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;

	}
	else if(c >= '@' && c <= 'O'){
		srcRct.y = 2*g::FONT_H;
		srcRct.x = ((int)c-64)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;
	}
	else if(c >= 'P' && c <= '_'){
		srcRct.y = 3*g::FONT_H;
		srcRct.x = ((int)c-80)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;
	}
	else if(c >= '\'' && c <= 'o'){
		srcRct.y = 4*g::FONT_H;
		srcRct.x = ((int)c-96)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;
	}
	else if(c >= 'p' && c <= '~'){
		srcRct.y = 5*g::FONT_H;
		srcRct.x = ((int)c-112)*g::FONT_W;
		srcRct.w = g::FONT_W;
		srcRct.h = g::FONT_H;
	}
	SDL_Rect dstRct;
	dstRct.x = x*g::FONT_W;
	dstRct.y = y*g::FONT_H;
	dstRct.w = g::FONT_W;
	dstRct.h = g::FONT_H;
  
	SDL_Rect rect = {dstRct.x, dstRct.y, dstRct.w, dstRct.h};
	SDL_SetRenderDrawColor(_renderer, cC.r, cC.g, cC.b, 255);
	SDL_RenderFillRect(_renderer, &rect);
	SDL_RenderCopy(_renderer, _font, &srcRct, &dstRct);
} // drawChar
	
void Clear(){
	SDL_SetRenderDrawColor(_renderer, 0, 0, 0, 255);
	SDL_RenderClear(_renderer);
} // Clear

void Flip(){
	SDL_RenderPresent(_renderer);
} // Flip

}
