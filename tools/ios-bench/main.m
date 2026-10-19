// A bare iOS app around zopro-bench: it runs the bench with the arguments
// it was launched with (from its Documents directory, where the dumps are)
// on a background thread, prints to the console devicectl attaches, and
// exits when the bench returns.
#import <UIKit/UIKit.h>

extern void zopro_bench_main(void);

@interface Scene : UIResponder <UIWindowSceneDelegate>
@property(strong, nonatomic) UIWindow *window;
@end

@implementation Scene
- (void)scene:(UIScene *)scene
    willConnectToSession:(UISceneSession *)session
                 options:(UISceneConnectionOptions *)options {
  self.window = [[UIWindow alloc] initWithWindowScene:(UIWindowScene *)scene];
  self.window.rootViewController = [UIViewController new];
  self.window.rootViewController.view.backgroundColor = UIColor.blackColor;
  [self.window makeKeyAndVisible];
  UIApplication.sharedApplication.idleTimerDisabled = YES;
  dispatch_async(dispatch_get_global_queue(QOS_CLASS_USER_INITIATED, 0), ^{
    zopro_bench_main();
    fflush(stdout);
    fflush(stderr);
    exit(0);
  });
}
@end

@interface App : UIResponder <UIApplicationDelegate>
@end

@implementation App
- (UISceneConfiguration *)application:(UIApplication *)application
    configurationForConnectingSceneSession:(UISceneSession *)session
                                   options:(UISceneConnectionOptions *)options {
  UISceneConfiguration *config = [[UISceneConfiguration alloc] initWithName:@"Bench"
                                                                sessionRole:session.role];
  config.delegateClass = [Scene class];
  return config;
}
@end

int main(int argc, char *argv[]) {
  setvbuf(stdout, NULL, _IOLBF, 0);
  @autoreleasepool {
    return UIApplicationMain(argc, argv, nil, NSStringFromClass([App class]));
  }
}
