/* automatically generated by rust-bindgen */
#[legacy_exports];

use core::libc::*;

type FcChar8 = c_uchar;
type FcChar16 = c_ushort;
type FcChar32 = c_uint;
type FcBool = c_int;

type enum__FcType = c_uint;
const FcTypeVoid: u32 = 0_u32;
const FcTypeInteger: u32 = 1_u32;
const FcTypeDouble: u32 = 2_u32;
const FcTypeString: u32 = 3_u32;
const FcTypeBool: u32 = 4_u32;
const FcTypeMatrix: u32 = 5_u32;
const FcTypeCharSet: u32 = 6_u32;
const FcTypeFTFace: u32 = 7_u32;
const FcTypeLangSet: u32 = 8_u32;

type FcType = enum__FcType;

struct struct__FcMatrix {
    xx: c_double,
    xy: c_double,
    yx: c_double,
    yy: c_double,
}

type FcMatrix = struct__FcMatrix;

type struct__FcCharSet = c_void;

type FcCharSet = struct__FcCharSet;

struct struct__FcObjectType {
    object: *c_char,
    _type: FcType,
}

type FcObjectType = struct__FcObjectType;

struct struct__FcConstant {
    name: *FcChar8,
    object: *c_char,
    value: c_int,
}

type FcConstant = struct__FcConstant;

type enum__FcResult = c_uint;
const FcResultMatch: u32 = 0_u32;
const FcResultNoMatch: u32 = 1_u32;
const FcResultTypeMismatch: u32 = 2_u32;
const FcResultNoId: u32 = 3_u32;
const FcResultOutOfMemory: u32 = 4_u32;

type FcResult = enum__FcResult;

type struct__FcPattern = c_void;

type FcPattern = struct__FcPattern;

type struct__FcLangSet = c_void;

type FcLangSet = struct__FcLangSet;

struct struct__FcValue {
    _type: FcType,
    u: union_unnamed1,
}

type FcValue = struct__FcValue;

struct struct__FcFontSet {
    nfont: c_int,
    sfont: c_int,
    fonts: **FcPattern,
}

type FcFontSet = struct__FcFontSet;

struct struct__FcObjectSet {
    nobject: c_int,
    sobject: c_int,
    objects: **c_char,
}

type FcObjectSet = struct__FcObjectSet;

type enum__FcMatchKind = c_uint;
const FcMatchPattern: u32 = 0_u32;
const FcMatchFont: u32 = 1_u32;
const FcMatchScan: u32 = 2_u32;

type FcMatchKind = enum__FcMatchKind;

type enum__FcLangResult = c_uint;
const FcLangEqual: u32 = 0_u32;
const FcLangDifferentCountry: u32 = 1_u32;
const FcLangDifferentTerritory: u32 = 1_u32;
const FcLangDifferentLang: u32 = 2_u32;

type FcLangResult = enum__FcLangResult;

type enum__FcSetName = c_uint;
const FcSetSystem: u32 = 0_u32;
const FcSetApplication: u32 = 1_u32;

type FcSetName = enum__FcSetName;

type struct__FcAtomic = c_void;

type FcAtomic = struct__FcAtomic;


type FcEndian = c_uint;
const FcEndianBig: u32 = 0_u32;
const FcEndianLittle: u32 = 1_u32;

type struct__FcConfig = c_void;

type FcConfig = struct__FcConfig;

type struct__FcGlobalCache = c_void;

type FcFileCache = struct__FcGlobalCache;

type struct__FcBlanks = c_void;

type FcBlanks = struct__FcBlanks;

type struct__FcStrList = c_void;

type FcStrList = struct__FcStrList;

type struct__FcStrSet = c_void;

type FcStrSet = struct__FcStrSet;

type struct__FcCache = c_void;

type FcCache = struct__FcCache;

type union_unnamed1 = c_void /* FIXME: union type */;

#[link_name="fontconfig"]
extern mod bindgen {

fn FcBlanksCreate() -> *FcBlanks;

fn FcBlanksDestroy(++b: *FcBlanks);

fn FcBlanksAdd(++b: *FcBlanks, ++ucs4: FcChar32) -> FcBool;

fn FcBlanksIsMember(++b: *FcBlanks, ++ucs4: FcChar32) -> FcBool;

fn FcCacheDir(++c: *FcCache) -> *FcChar8;

fn FcCacheCopySet(++c: *FcCache) -> *FcFontSet;

fn FcCacheSubdir(++c: *FcCache, ++i: c_int) -> *FcChar8;

fn FcCacheNumSubdir(++c: *FcCache) -> c_int;

fn FcCacheNumFont(++c: *FcCache) -> c_int;

fn FcDirCacheUnlink(++dir: *FcChar8, ++config: *FcConfig) -> FcBool;

fn FcDirCacheValid(++cache_file: *FcChar8) -> FcBool;

fn FcConfigHome() -> *FcChar8;

fn FcConfigEnableHome(++enable: FcBool) -> FcBool;

fn FcConfigFilename(++url: *FcChar8) -> *FcChar8;

fn FcConfigCreate() -> *FcConfig;

fn FcConfigReference(++config: *FcConfig) -> *FcConfig;

fn FcConfigDestroy(++config: *FcConfig);

fn FcConfigSetCurrent(++config: *FcConfig) -> FcBool;

fn FcConfigGetCurrent() -> *FcConfig;

fn FcConfigUptoDate(++config: *FcConfig) -> FcBool;

fn FcConfigBuildFonts(++config: *FcConfig) -> FcBool;

fn FcConfigGetFontDirs(++config: *FcConfig) -> *FcStrList;

fn FcConfigGetConfigDirs(++config: *FcConfig) -> *FcStrList;

fn FcConfigGetConfigFiles(++config: *FcConfig) -> *FcStrList;

fn FcConfigGetCache(++config: *FcConfig) -> *FcChar8;

fn FcConfigGetBlanks(++config: *FcConfig) -> *FcBlanks;

fn FcConfigGetCacheDirs(++config: *FcConfig) -> *FcStrList;

fn FcConfigGetRescanInterval(++config: *FcConfig) -> c_int;

fn FcConfigSetRescanInterval(++config: *FcConfig, ++rescanInterval: c_int) -> FcBool;

fn FcConfigGetFonts(++config: *FcConfig, ++set: FcSetName) -> *FcFontSet;

fn FcConfigAppFontAddFile(++config: *FcConfig, ++file: *FcChar8) -> FcBool;

fn FcConfigAppFontAddDir(++config: *FcConfig, ++dir: *FcChar8) -> FcBool;

fn FcConfigAppFontClear(++config: *FcConfig);

fn FcConfigSubstituteWithPat(++config: *FcConfig, ++p: *FcPattern, ++p_pat: *FcPattern, ++kind: FcMatchKind) -> FcBool;

fn FcConfigSubstitute(++config: *FcConfig, ++p: *FcPattern, ++kind: FcMatchKind) -> FcBool;

fn FcCharSetCreate() -> *FcCharSet;

fn FcCharSetNew() -> *FcCharSet;

fn FcCharSetDestroy(++fcs: *FcCharSet);

fn FcCharSetAddChar(++fcs: *FcCharSet, ++ucs4: FcChar32) -> FcBool;

fn FcCharSetCopy(++src: *FcCharSet) -> *FcCharSet;

fn FcCharSetEqual(++a: *FcCharSet, ++b: *FcCharSet) -> FcBool;

fn FcCharSetIntersect(++a: *FcCharSet, ++b: *FcCharSet) -> *FcCharSet;

fn FcCharSetUnion(++a: *FcCharSet, ++b: *FcCharSet) -> *FcCharSet;

fn FcCharSetSubtract(++a: *FcCharSet, ++b: *FcCharSet) -> *FcCharSet;

fn FcCharSetMerge(++a: *FcCharSet, ++b: *FcCharSet, ++changed: *FcBool) -> FcBool;

fn FcCharSetHasChar(++fcs: *FcCharSet, ++ucs4: FcChar32) -> FcBool;

fn FcCharSetCount(++a: *FcCharSet) -> FcChar32;

fn FcCharSetIntersectCount(++a: *FcCharSet, ++b: *FcCharSet) -> FcChar32;

fn FcCharSetSubtractCount(++a: *FcCharSet, ++b: *FcCharSet) -> FcChar32;

fn FcCharSetIsSubset(++a: *FcCharSet, ++b: *FcCharSet) -> FcBool;

fn FcCharSetFirstPage(++a: *FcCharSet, ++map: *FcChar32, ++next: *FcChar32) -> FcChar32;

fn FcCharSetNextPage(++a: *FcCharSet, ++map: *FcChar32, ++next: *FcChar32) -> FcChar32;

fn FcCharSetCoverage(++a: *FcCharSet, ++page: FcChar32, ++result: *FcChar32) -> FcChar32;

fn FcValuePrint(++v: FcValue);

fn FcPatternPrint(++p: *FcPattern);

fn FcFontSetPrint(++s: *FcFontSet);

fn FcDefaultSubstitute(++pattern: *FcPattern);

fn FcFileIsDir(++file: *FcChar8) -> FcBool;

fn FcFileScan(++set: *FcFontSet, ++dirs: *FcStrSet, ++cache: *FcFileCache, ++blanks: *FcBlanks, ++file: *FcChar8, ++force: FcBool) -> FcBool;

fn FcDirScan(++set: *FcFontSet, ++dirs: *FcStrSet, ++cache: *FcFileCache, ++blanks: *FcBlanks, ++dir: *FcChar8, ++force: FcBool) -> FcBool;

fn FcDirSave(++set: *FcFontSet, ++dirs: *FcStrSet, ++dir: *FcChar8) -> FcBool;

fn FcDirCacheLoad(++dir: *FcChar8, ++config: *FcConfig, ++cache_file: **FcChar8) -> *FcCache;

fn FcDirCacheRead(++dir: *FcChar8, ++force: FcBool, ++config: *FcConfig) -> *FcCache;

//fn FcDirCacheLoadFile(++cache_file: *FcChar8, ++file_stat: *struct_stat) -> *FcCache;

fn FcDirCacheUnload(++cache: *FcCache);

fn FcFreeTypeQuery(++file: *FcChar8, ++id: c_int, ++blanks: *FcBlanks, ++count: *c_int) -> *FcPattern;

fn FcFontSetCreate() -> *FcFontSet;

fn FcFontSetDestroy(++s: *FcFontSet);

fn FcFontSetAdd(++s: *FcFontSet, ++font: *FcPattern) -> FcBool;

fn FcInitLoadConfig() -> *FcConfig;

fn FcInitLoadConfigAndFonts() -> *FcConfig;

fn FcInit() -> FcBool;

fn FcFini();

fn FcGetVersion() -> c_int;

fn FcInitReinitialize() -> FcBool;

fn FcInitBringUptoDate() -> FcBool;

fn FcGetLangs() -> *FcStrSet;

fn FcLangGetCharSet(++lang: *FcChar8) -> *FcCharSet;

fn FcLangSetCreate() -> *FcLangSet;

fn FcLangSetDestroy(++ls: *FcLangSet);

fn FcLangSetCopy(++ls: *FcLangSet) -> *FcLangSet;

fn FcLangSetAdd(++ls: *FcLangSet, ++lang: *FcChar8) -> FcBool;

fn FcLangSetHasLang(++ls: *FcLangSet, ++lang: *FcChar8) -> FcLangResult;

fn FcLangSetCompare(++lsa: *FcLangSet, ++lsb: *FcLangSet) -> FcLangResult;

fn FcLangSetContains(++lsa: *FcLangSet, ++lsb: *FcLangSet) -> FcBool;

fn FcLangSetEqual(++lsa: *FcLangSet, ++lsb: *FcLangSet) -> FcBool;

fn FcLangSetHash(++ls: *FcLangSet) -> FcChar32;

fn FcLangSetGetLangs(++ls: *FcLangSet) -> *FcStrSet;

fn FcObjectSetCreate() -> *FcObjectSet;

fn FcObjectSetAdd(++os: *FcObjectSet, ++object: *c_char) -> FcBool;

fn FcObjectSetDestroy(++os: *FcObjectSet);

//fn FcObjectSetVaBuild(++first: *c_char, ++va: *__va_list_tag) -> *FcObjectSet;

fn FcObjectSetBuild(++first: *c_char/* FIXME: variadic function */) -> *FcObjectSet;

fn FcFontSetList(++config: *FcConfig, ++sets: **FcFontSet, ++nsets: c_int, ++p: *FcPattern, ++os: *FcObjectSet) -> *FcFontSet;

fn FcFontList(++config: *FcConfig, ++p: *FcPattern, ++os: *FcObjectSet) -> *FcFontSet;

fn FcAtomicCreate(++file: *FcChar8) -> *FcAtomic;

fn FcAtomicLock(++atomic: *FcAtomic) -> FcBool;

fn FcAtomicNewFile(++atomic: *FcAtomic) -> *FcChar8;

fn FcAtomicOrigFile(++atomic: *FcAtomic) -> *FcChar8;

fn FcAtomicReplaceOrig(++atomic: *FcAtomic) -> FcBool;

fn FcAtomicDeleteNew(++atomic: *FcAtomic);

fn FcAtomicUnlock(++atomic: *FcAtomic);

fn FcAtomicDestroy(++atomic: *FcAtomic);

fn FcFontSetMatch(++config: *FcConfig, ++sets: **FcFontSet, ++nsets: c_int, ++p: *FcPattern, ++result: *FcResult) -> *FcPattern;

fn FcFontMatch(++config: *FcConfig, ++p: *FcPattern, ++result: *FcResult) -> *FcPattern;

fn FcFontRenderPrepare(++config: *FcConfig, ++pat: *FcPattern, ++font: *FcPattern) -> *FcPattern;

fn FcFontSetSort(++config: *FcConfig, ++sets: **FcFontSet, ++nsets: c_int, ++p: *FcPattern, ++trim: FcBool, ++csp: **FcCharSet, ++result: *FcResult) -> *FcFontSet;

fn FcFontSort(++config: *FcConfig, ++p: *FcPattern, ++trim: FcBool, ++csp: **FcCharSet, ++result: *FcResult) -> *FcFontSet;

fn FcFontSetSortDestroy(++fs: *FcFontSet);

fn FcMatrixCopy(++mat: *FcMatrix) -> *FcMatrix;

fn FcMatrixEqual(++mat1: *FcMatrix, ++mat2: *FcMatrix) -> FcBool;

fn FcMatrixMultiply(++result: *FcMatrix, ++a: *FcMatrix, ++b: *FcMatrix);

fn FcMatrixRotate(++m: *FcMatrix, ++c: c_double, ++s: c_double);

fn FcMatrixScale(++m: *FcMatrix, ++sx: c_double, ++sy: c_double);

fn FcMatrixShear(++m: *FcMatrix, ++sh: c_double, ++sv: c_double);

fn FcNameRegisterObjectTypes(++types: *FcObjectType, ++ntype: c_int) -> FcBool;

fn FcNameUnregisterObjectTypes(++types: *FcObjectType, ++ntype: c_int) -> FcBool;

fn FcNameGetObjectType(++object: *c_char) -> *FcObjectType;

fn FcNameRegisterConstants(++consts: *FcConstant, ++nconsts: c_int) -> FcBool;

fn FcNameUnregisterConstants(++consts: *FcConstant, ++nconsts: c_int) -> FcBool;

fn FcNameGetConstant(++string: *FcChar8) -> *FcConstant;

fn FcNameConstant(++string: *FcChar8, ++result: *c_int) -> FcBool;

fn FcNameParse(++name: *FcChar8) -> *FcPattern;

fn FcNameUnparse(++pat: *FcPattern) -> *FcChar8;

fn FcPatternCreate() -> *FcPattern;

fn FcPatternDuplicate(++p: *FcPattern) -> *FcPattern;

fn FcPatternReference(++p: *FcPattern);

fn FcPatternFilter(++p: *FcPattern, ++os: *FcObjectSet) -> *FcPattern;

fn FcValueDestroy(++v: FcValue);

fn FcValueEqual(++va: FcValue, ++vb: FcValue) -> FcBool;

fn FcValueSave(++v: FcValue) -> FcValue;

fn FcPatternDestroy(++p: *FcPattern);

fn FcPatternEqual(++pa: *FcPattern, ++pb: *FcPattern) -> FcBool;

fn FcPatternEqualSubset(++pa: *FcPattern, ++pb: *FcPattern, ++os: *FcObjectSet) -> FcBool;

fn FcPatternHash(++p: *FcPattern) -> FcChar32;

fn FcPatternAdd(++p: *FcPattern, ++object: *c_char, ++value: FcValue, ++append: FcBool) -> FcBool;

fn FcPatternAddWeak(++p: *FcPattern, ++object: *c_char, ++value: FcValue, ++append: FcBool) -> FcBool;

fn FcPatternGet(++p: *FcPattern, ++object: *c_char, ++id: c_int, ++v: *FcValue) -> FcResult;

fn FcPatternDel(++p: *FcPattern, ++object: *c_char) -> FcBool;

fn FcPatternRemove(++p: *FcPattern, ++object: *c_char, ++id: c_int) -> FcBool;

fn FcPatternAddInteger(++p: *FcPattern, ++object: *c_char, ++i: c_int) -> FcBool;

fn FcPatternAddDouble(++p: *FcPattern, ++object: *c_char, ++d: c_double) -> FcBool;

fn FcPatternAddString(++p: *FcPattern, ++object: *c_char, ++s: *FcChar8) -> FcBool;

fn FcPatternAddMatrix(++p: *FcPattern, ++object: *c_char, ++s: *FcMatrix) -> FcBool;

fn FcPatternAddCharSet(++p: *FcPattern, ++object: *c_char, ++c: *FcCharSet) -> FcBool;

fn FcPatternAddBool(++p: *FcPattern, ++object: *c_char, ++b: FcBool) -> FcBool;

fn FcPatternAddLangSet(++p: *FcPattern, ++object: *c_char, ++ls: *FcLangSet) -> FcBool;

fn FcPatternGetInteger(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++i: *c_int) -> FcResult;

fn FcPatternGetDouble(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++d: *c_double) -> FcResult;

fn FcPatternGetString(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++s: **FcChar8) -> FcResult;

fn FcPatternGetMatrix(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++s: **FcMatrix) -> FcResult;

fn FcPatternGetCharSet(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++c: **FcCharSet) -> FcResult;

fn FcPatternGetBool(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++b: *FcBool) -> FcResult;

fn FcPatternGetLangSet(++p: *FcPattern, ++object: *c_char, ++n: c_int, ++ls: **FcLangSet) -> FcResult;

//fn FcPatternVaBuild(++p: *FcPattern, ++va: *__va_list_tag) -> *FcPattern;

fn FcPatternBuild(++p: *FcPattern/* FIXME: variadic function */) -> *FcPattern;

fn FcPatternFormat(++pat: *FcPattern, ++format: *FcChar8) -> *FcChar8;

fn FcStrCopy(++s: *FcChar8) -> *FcChar8;

fn FcStrCopyFilename(++s: *FcChar8) -> *FcChar8;

fn FcStrPlus(++s1: *FcChar8, ++s2: *FcChar8) -> *FcChar8;

fn FcStrFree(++s: *FcChar8);

fn FcStrDowncase(++s: *FcChar8) -> *FcChar8;

fn FcStrCmpIgnoreCase(++s1: *FcChar8, ++s2: *FcChar8) -> c_int;

fn FcStrCmp(++s1: *FcChar8, ++s2: *FcChar8) -> c_int;

fn FcStrStrIgnoreCase(++s1: *FcChar8, ++s2: *FcChar8) -> *FcChar8;

fn FcStrStr(++s1: *FcChar8, ++s2: *FcChar8) -> *FcChar8;

fn FcUtf8ToUcs4(++src_orig: *FcChar8, ++dst: *FcChar32, ++len: c_int) -> c_int;

fn FcUtf8Len(++string: *FcChar8, ++len: c_int, ++nchar: *c_int, ++wchar: *c_int) -> FcBool;

fn FcUcs4ToUtf8(++ucs4: FcChar32, ++dest: *FcChar8) -> c_int;

fn FcUtf16ToUcs4(++src_orig: *FcChar8, ++endian: FcEndian, ++dst: *FcChar32, ++len: c_int) -> c_int;

fn FcUtf16Len(++string: *FcChar8, ++endian: FcEndian, ++len: c_int, ++nchar: *c_int, ++wchar: *c_int) -> FcBool;

fn FcStrDirname(++file: *FcChar8) -> *FcChar8;

fn FcStrBasename(++file: *FcChar8) -> *FcChar8;

fn FcStrSetCreate() -> *FcStrSet;

fn FcStrSetMember(++set: *FcStrSet, ++s: *FcChar8) -> FcBool;

fn FcStrSetEqual(++sa: *FcStrSet, ++sb: *FcStrSet) -> FcBool;

fn FcStrSetAdd(++set: *FcStrSet, ++s: *FcChar8) -> FcBool;

fn FcStrSetAddFilename(++set: *FcStrSet, ++s: *FcChar8) -> FcBool;

fn FcStrSetDel(++set: *FcStrSet, ++s: *FcChar8) -> FcBool;

fn FcStrSetDestroy(++set: *FcStrSet);

fn FcStrListCreate(++set: *FcStrSet) -> *FcStrList;

fn FcStrListNext(++list: *FcStrList) -> *FcChar8;

fn FcStrListDone(++list: *FcStrList);

fn FcConfigParseAndLoad(++config: *FcConfig, ++file: *FcChar8, ++complain: FcBool) -> FcBool;

}
