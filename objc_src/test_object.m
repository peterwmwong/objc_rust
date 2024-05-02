#include "test_object.h"

@implementation TestObject
-(uint32_t)getValue:(uint32_t)v {
    NSLog(@"TestObject getReturnCode called!");
    return v + 123;
}
@end
