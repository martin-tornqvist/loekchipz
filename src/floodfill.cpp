#include "floodfill.hpp"
#include "geometry.hpp"

#include <iostream>

const int FLOOD_VALUE_UNREACHED = -1;

A2<int> flood_fill(P p0, P *p1, A2<bool> blocked, int *travel_lmt){

        
        A2<int> result(blocked.dims(), FLOOD_VALUE_UNREACHED);
        
        result.set_at_p(p0, 0);
        
        // Vector of positions to travel to
        // In the worst case we need to visit every position, reserve elements
        // for this to avoid lots of expensive resizing
        std::vector<P> positions(result.size());

        // Instead of removing evaluated positions from the vector, we track
        // which index to try next (cheaper than erasing front elements).
        int next_p_idx = 0;

        bool path_exists = true;

        P p = p0;

        int v = 0;

        bool is_at_tgt = false;

        bool done = false;

        while( !done ) {
                // Flood around the current position, and add those positions to the
                // list of positions to travel to
                
                for(auto const & d : dir_utils::directions ) {
                        P new_p = p;
                        new_p.x += d.x;
                        new_p.y += d.y;
                
                        // Not inside the bounds?
                        if( !result.is_p_inside(new_p) ) {
                                continue;
                        }

                        // blocked?
                        if( blocked.copy_from_p(new_p) ) {
                                continue;
                        }
                        
                        // Already visited?
                        if( result.at_p(new_p) != FLOOD_VALUE_UNREACHED ) {
                                continue;
                        }
                        
                        // this is the origin?
                        if( new_p == p0) {
                                continue;
                        }

                        v = result.copy_from_p(p);

                        if( travel_lmt == nullptr || v < *travel_lmt ) {
                                result.set_at_p(new_p, v + 1);
                        }
                        
                        // Reached the target?
                        if( p1 != nullptr && new_p == *p1 ) {
                                is_at_tgt = true;
                                break;
                        }

                        if( p1 == nullptr || is_at_tgt) {
                                
                                positions.push_back(new_p);
                        }
                        
                } // offset loop

                if(p1 != nullptr) {
                        if( (int)positions.size() == next_p_idx ) {
                                path_exists = false;
                        }
                        
                        if( is_at_tgt || !path_exists ) {
                                done = true;
                        }
                }
                else if( (int)positions.size() == next_p_idx ) {
                        done = true;
                }

                if( travel_lmt != nullptr &&  (v == *travel_lmt) ) {
                        done = true;
                }

                if( p1 != nullptr || !is_at_tgt ) {
                        if( (int)positions.size() == next_p_idx ) {
                                // no more positions to evaluate
                                path_exists = false;
                        }
                        else {
                                // there are more positions to evaluate
                                p = positions[next_p_idx];
                                next_p_idx += 1;
                        }                                                                            
                
                        
                        
                }
                
        }// flood_loop

   
        
        return result;
}
