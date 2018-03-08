#include "io.h"

int main(){
	
	io::Init();

	for(int i = 0; i < 1000; ++i){
		io::Clear();
		io::drawChar('@', 10, 10, cColor{100, 30, 30});
		io::Flip();
	}
	io::Cleanup();
	return 0;
}
