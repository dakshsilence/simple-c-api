#ifdef __cplusplus
#import "react-native-simple.h"
#endif

#ifdef RCT_NEW_ARCH_ENABLED
#import "RNSimpleSpec.h"

@interface Simple : NSObject <NativeSimpleSpec>
#else
#import <React/RCTBridgeModule.h>

@interface Simple : NSObject <RCTBridgeModule>
#endif

@end
