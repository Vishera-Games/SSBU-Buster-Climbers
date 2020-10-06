# SSBU-Buster-Climbers
Requested source code for my SSBU Buster Climbers mod


The sword trail function for popo is:
AFTER_IMAGE4_ON_arg29(84654574715 as u64, 82355625409 as u64, 12, hash40("haver"), 0, 2.5, 0, hash40("haver"), 0, 21.0, 0, true, LUA_VOID, hash40("haver"), 0, 0, 0, 0, 0, 0, 1, 0, EFFECT_AXIS_X, 0, TRAIL_BLEND_ALPHA, 101, TRAIL_CULL_NONE, 1.29999995, 0.100000001)

The function with args I know:
AFTER_IMAGE4_ON_arg29(effectHash1?, effectHash2?, trail intensity, bone1, x1, y1, z1, bone2, x2, y2, z2, unk(true), unk(void), bone3, x3, y3, z3, xrot, yrot, zrot, size, unk......

The third set of x, y, z coordinates do not seem to do anything, the top and bottom positions of the sword trail are controlled by the first 2 sets of bones and coordinates. I do not know if the bone3, rotation, or size args do anything as I don't see why you would use them for a sword trail
