Canvas size: 1920x1080
Fullscreen size: 1440x900

Subject box size: 920x728

% subject box size on canvas: ~48% x ~67%
% subject box size on fullscreen: ~64% x ~81%
% size difference: ~16% x ~14%

Desired canvas size: 920x728
Desired fullscreen width;
920 / 1920 * 1440 = 690
Desired fullscreen height;
728 / 1080 * 900 = 606.66... = ~606.7

Width workings;
width = 920/1920*screen_width()

Height scale factor;
height = 728/1080*screen_height()

-----------------------------------------------

Actual dimensions: 690x606.6666
Issues;
- Improper width
Solution;
- Stop being stupid. Can't measure width with a missized title element...
- Fix title element scaling to also be in accordance with reality