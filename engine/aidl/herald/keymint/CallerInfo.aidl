package herald.keymint;

@RustDerive(Clone=true)
parcelable CallerInfo {
     long uid;
     String sid;
     long pid;
}
