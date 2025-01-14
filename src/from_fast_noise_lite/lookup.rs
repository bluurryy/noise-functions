use super::{Table2, Table3};

macro_rules! const_assert {
    ($($tt:tt)*) => {
        const _: () = assert!($($tt)*);
    };
}

const_assert!(i32::BITS <= usize::BITS, "We cast i32 to usize for indexing.");

#[cfg(feature = "nightly-simd")]
mod entry {
    use core::simd::{f32x2, f32x4, LaneCount, Simd, SupportedLaneCount};

    const_assert!(
        core::mem::size_of::<f32x2>() == 8 && core::mem::align_of::<f32x2>() <= 8,
        "We assume this for the lookup code. Please let me know if this assertion ever fails."
    );

    const_assert!(
        core::mem::size_of::<f32x4>() == 16 && core::mem::align_of::<f32x4>() <= 16,
        "We assume this for the lookup code. Please let me know if this assertion ever fails."
    );

    #[repr(transparent)]
    pub(crate) struct Entry<const N: usize>(pub Simd<f32, N>)
    where
        LaneCount<N>: SupportedLaneCount;

    impl<const N: usize> Entry<N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        pub(crate) fn as_array(&self) -> &[f32; N] {
            self.0.as_array()
        }
    }

    pub(crate) const fn entry2(x: f32, y: f32) -> Entry<2> {
        Entry(f32x2::from_array([x, y]))
    }

    pub(crate) const fn entry3(x: f32, y: f32, z: f32) -> Entry<4> {
        Entry(f32x4::from_array([x, y, z, 0.0]))
    }
}

#[cfg(not(feature = "nightly-simd"))]
mod entry {
    #[repr(transparent)]
    pub(crate) struct Entry<const N: usize>([f32; N]);

    impl<const N: usize> Entry<N> {
        pub(crate) fn as_array(&self) -> &[f32; N] {
            &self.0
        }
    }

    pub(crate) const fn entry2(x: f32, y: f32) -> Entry<2> {
        Entry([x, y])
    }

    pub(crate) const fn entry3(x: f32, y: f32, z: f32) -> Entry<4> {
        Entry([x, y, z, 0.0])
    }
}

pub(crate) use entry::{entry2, entry3, Entry};

pub(crate) const RAND_VECS_2D: Table2<256> = Table2::new([
    entry2(-0.2700222198, -0.9628540911),
    entry2(0.3863092627, -0.9223693152),
    entry2(0.04444859006, -0.999011673),
    entry2(-0.5992523158, -0.8005602176),
    entry2(-0.7819280288, 0.6233687174),
    entry2(0.9464672271, 0.3227999196),
    entry2(-0.6514146797, -0.7587218957),
    entry2(0.9378472289, 0.347048376),
    entry2(-0.8497875957, -0.5271252623),
    entry2(-0.879042592, 0.4767432447),
    entry2(-0.892300288, -0.4514423508),
    entry2(-0.379844434, -0.9250503802),
    entry2(-0.9951650832, 0.0982163789),
    entry2(0.7724397808, -0.6350880136),
    entry2(0.7573283322, -0.6530343002),
    entry2(-0.9928004525, -0.119780055),
    entry2(-0.0532665713, 0.9985803285),
    entry2(0.9754253726, -0.2203300762),
    entry2(-0.7665018163, 0.6422421394),
    entry2(0.991636706, 0.1290606184),
    entry2(-0.994696838, 0.1028503788),
    entry2(-0.5379205513, -0.84299554),
    entry2(0.5022815471, -0.8647041387),
    entry2(0.4559821461, -0.8899889226),
    entry2(-0.8659131224, -0.5001944266),
    entry2(0.0879458407, -0.9961252577),
    entry2(-0.5051684983, 0.8630207346),
    entry2(0.7753185226, -0.6315704146),
    entry2(-0.6921944612, 0.7217110418),
    entry2(-0.5191659449, -0.8546734591),
    entry2(0.8978622882, -0.4402764035),
    entry2(-0.1706774107, 0.9853269617),
    entry2(-0.9353430106, -0.3537420705),
    entry2(-0.9992404798, 0.03896746794),
    entry2(-0.2882064021, -0.9575683108),
    entry2(-0.9663811329, 0.2571137995),
    entry2(-0.8759714238, -0.4823630009),
    entry2(-0.8303123018, -0.5572983775),
    entry2(0.05110133755, -0.9986934731),
    entry2(-0.8558373281, -0.5172450752),
    entry2(0.09887025282, 0.9951003332),
    entry2(0.9189016087, 0.3944867976),
    entry2(-0.2439375892, -0.9697909324),
    entry2(-0.8121409387, -0.5834613061),
    entry2(-0.9910431363, 0.1335421355),
    entry2(0.8492423985, -0.5280031709),
    entry2(-0.9717838994, -0.2358729591),
    entry2(0.9949457207, 0.1004142068),
    entry2(0.6241065508, -0.7813392434),
    entry2(0.662910307, 0.7486988212),
    entry2(-0.7197418176, 0.6942418282),
    entry2(-0.8143370775, -0.5803922158),
    entry2(0.104521054, -0.9945226741),
    entry2(-0.1065926113, -0.9943027784),
    entry2(0.445799684, -0.8951327509),
    entry2(0.105547406, 0.9944142724),
    entry2(-0.992790267, 0.1198644477),
    entry2(-0.8334366408, 0.552615025),
    entry2(0.9115561563, -0.4111755999),
    entry2(0.8285544909, -0.5599084351),
    entry2(0.7217097654, -0.6921957921),
    entry2(0.4940492677, -0.8694339084),
    entry2(-0.3652321272, -0.9309164803),
    entry2(-0.9696606758, 0.2444548501),
    entry2(0.08925509731, -0.996008799),
    entry2(0.5354071276, -0.8445941083),
    entry2(-0.1053576186, 0.9944343981),
    entry2(-0.9890284586, 0.1477251101),
    entry2(0.004856104961, 0.9999882091),
    entry2(0.9885598478, 0.1508291331),
    entry2(0.9286129562, -0.3710498316),
    entry2(-0.5832393863, -0.8123003252),
    entry2(0.3015207509, 0.9534596146),
    entry2(-0.9575110528, 0.2883965738),
    entry2(0.9715802154, -0.2367105511),
    entry2(0.229981792, 0.9731949318),
    entry2(0.955763816, -0.2941352207),
    entry2(0.740956116, 0.6715534485),
    entry2(-0.9971513787, -0.07542630764),
    entry2(0.6905710663, -0.7232645452),
    entry2(-0.290713703, -0.9568100872),
    entry2(0.5912777791, -0.8064679708),
    entry2(-0.9454592212, -0.325740481),
    entry2(0.6664455681, 0.74555369),
    entry2(0.6236134912, 0.7817328275),
    entry2(0.9126993851, -0.4086316587),
    entry2(-0.8191762011, 0.5735419353),
    entry2(-0.8812745759, -0.4726046147),
    entry2(0.9953313627, 0.09651672651),
    entry2(0.9855650846, -0.1692969699),
    entry2(-0.8495980887, 0.5274306472),
    entry2(0.6174853946, -0.7865823463),
    entry2(0.8508156371, 0.52546432),
    entry2(0.9985032451, -0.05469249926),
    entry2(0.1971371563, -0.9803759185),
    entry2(0.6607855748, -0.7505747292),
    entry2(-0.03097494063, 0.9995201614),
    entry2(-0.6731660801, 0.739491331),
    entry2(-0.7195018362, -0.6944905383),
    entry2(0.9727511689, 0.2318515979),
    entry2(0.9997059088, -0.0242506907),
    entry2(0.4421787429, -0.8969269532),
    entry2(0.9981350961, -0.061043673),
    entry2(-0.9173660799, -0.3980445648),
    entry2(-0.8150056635, -0.5794529907),
    entry2(-0.8789331304, 0.4769450202),
    entry2(0.0158605829, 0.999874213),
    entry2(-0.8095464474, 0.5870558317),
    entry2(-0.9165898907, -0.3998286786),
    entry2(-0.8023542565, 0.5968480938),
    entry2(-0.5176737917, 0.8555780767),
    entry2(-0.8154407307, -0.5788405779),
    entry2(0.4022010347, -0.9155513791),
    entry2(-0.9052556868, -0.4248672045),
    entry2(0.7317445619, 0.6815789728),
    entry2(-0.5647632201, -0.8252529947),
    entry2(-0.8403276335, -0.5420788397),
    entry2(-0.9314281527, 0.363925262),
    entry2(0.5238198472, 0.8518290719),
    entry2(0.7432803869, -0.6689800195),
    entry2(-0.985371561, -0.1704197369),
    entry2(0.4601468731, 0.88784281),
    entry2(0.825855404, 0.5638819483),
    entry2(0.6182366099, 0.7859920446),
    entry2(0.8331502863, -0.553046653),
    entry2(0.1500307506, 0.9886813308),
    entry2(-0.662330369, -0.7492119075),
    entry2(-0.668598664, 0.743623444),
    entry2(0.7025606278, 0.7116238924),
    entry2(-0.5419389763, -0.8404178401),
    entry2(-0.3388616456, 0.9408362159),
    entry2(0.8331530315, 0.5530425174),
    entry2(-0.2989720662, -0.9542618632),
    entry2(0.2638522993, 0.9645630949),
    entry2(0.124108739, -0.9922686234),
    entry2(-0.7282649308, -0.6852956957),
    entry2(0.6962500149, 0.7177993569),
    entry2(-0.9183535368, 0.3957610156),
    entry2(-0.6326102274, -0.7744703352),
    entry2(-0.9331891859, -0.359385508),
    entry2(-0.1153779357, -0.9933216659),
    entry2(0.9514974788, -0.3076565421),
    entry2(-0.08987977445, -0.9959526224),
    entry2(0.6678496916, 0.7442961705),
    entry2(0.7952400393, -0.6062947138),
    entry2(-0.6462007402, -0.7631674805),
    entry2(-0.2733598753, 0.9619118351),
    entry2(0.9669590226, -0.254931851),
    entry2(-0.9792894595, 0.2024651934),
    entry2(-0.5369502995, -0.8436138784),
    entry2(-0.270036471, -0.9628500944),
    entry2(-0.6400277131, 0.7683518247),
    entry2(-0.7854537493, -0.6189203566),
    entry2(0.06005905383, -0.9981948257),
    entry2(-0.02455770378, 0.9996984141),
    entry2(-0.65983623, 0.751409442),
    entry2(-0.6253894466, -0.7803127835),
    entry2(-0.6210408851, -0.7837781695),
    entry2(0.8348888491, 0.5504185768),
    entry2(-0.1592275245, 0.9872419133),
    entry2(0.8367622488, 0.5475663786),
    entry2(-0.8675753916, -0.4973056806),
    entry2(-0.2022662628, -0.9793305667),
    entry2(0.9399189937, 0.3413975472),
    entry2(0.9877404807, -0.1561049093),
    entry2(-0.9034455656, 0.4287028224),
    entry2(0.1269804218, -0.9919052235),
    entry2(-0.3819600854, 0.924178821),
    entry2(0.9754625894, 0.2201652486),
    entry2(-0.3204015856, -0.9472818081),
    entry2(-0.9874760884, 0.1577687387),
    entry2(0.02535348474, -0.9996785487),
    entry2(0.4835130794, -0.8753371362),
    entry2(-0.2850799925, -0.9585037287),
    entry2(-0.06805516006, -0.99768156),
    entry2(-0.7885244045, -0.6150034663),
    entry2(0.3185392127, -0.9479096845),
    entry2(0.8880043089, 0.4598351306),
    entry2(0.6476921488, -0.7619021462),
    entry2(0.9820241299, 0.1887554194),
    entry2(0.9357275128, -0.3527237187),
    entry2(-0.8894895414, 0.4569555293),
    entry2(0.7922791302, 0.6101588153),
    entry2(0.7483818261, 0.6632681526),
    entry2(-0.7288929755, -0.6846276581),
    entry2(0.8729032783, -0.4878932944),
    entry2(0.8288345784, 0.5594937369),
    entry2(0.08074567077, 0.9967347374),
    entry2(0.9799148216, -0.1994165048),
    entry2(-0.580730673, -0.8140957471),
    entry2(-0.4700049791, -0.8826637636),
    entry2(0.2409492979, 0.9705377045),
    entry2(0.9437816757, -0.3305694308),
    entry2(-0.8927998638, -0.4504535528),
    entry2(-0.8069622304, 0.5906030467),
    entry2(0.06258973166, 0.9980393407),
    entry2(-0.9312597469, 0.3643559849),
    entry2(0.5777449785, 0.8162173362),
    entry2(-0.3360095855, -0.941858566),
    entry2(0.697932075, -0.7161639607),
    entry2(-0.002008157227, -0.9999979837),
    entry2(-0.1827294312, -0.9831632392),
    entry2(-0.6523911722, 0.7578824173),
    entry2(-0.4302626911, -0.9027037258),
    entry2(-0.9985126289, -0.05452091251),
    entry2(-0.01028102172, -0.9999471489),
    entry2(-0.4946071129, 0.8691166802),
    entry2(-0.2999350194, 0.9539596344),
    entry2(0.8165471961, 0.5772786819),
    entry2(0.2697460475, 0.962931498),
    entry2(-0.7306287391, -0.6827749597),
    entry2(-0.7590952064, -0.6509796216),
    entry2(-0.907053853, 0.4210146171),
    entry2(-0.5104861064, -0.8598860013),
    entry2(0.8613350597, 0.5080373165),
    entry2(0.5007881595, -0.8655698812),
    entry2(-0.654158152, 0.7563577938),
    entry2(-0.8382755311, -0.545246856),
    entry2(0.6940070834, 0.7199681717),
    entry2(0.06950936031, 0.9975812994),
    entry2(0.1702942185, -0.9853932612),
    entry2(0.2695973274, 0.9629731466),
    entry2(0.5519612192, -0.8338697815),
    entry2(0.225657487, -0.9742067022),
    entry2(0.4215262855, -0.9068161835),
    entry2(0.4881873305, -0.8727388672),
    entry2(-0.3683854996, -0.9296731273),
    entry2(-0.9825390578, 0.1860564427),
    entry2(0.81256471, 0.5828709909),
    entry2(0.3196460933, -0.9475370046),
    entry2(0.9570913859, 0.2897862643),
    entry2(-0.6876655497, -0.7260276109),
    entry2(-0.9988770922, -0.047376731),
    entry2(-0.1250179027, 0.992154486),
    entry2(-0.8280133617, 0.560708367),
    entry2(0.9324863769, -0.3612051451),
    entry2(0.6394653183, 0.7688199442),
    entry2(-0.01623847064, -0.9998681473),
    entry2(-0.9955014666, -0.09474613458),
    entry2(-0.81453315, 0.580117012),
    entry2(0.4037327978, -0.9148769469),
    entry2(0.9944263371, 0.1054336766),
    entry2(-0.1624711654, 0.9867132919),
    entry2(-0.9949487814, -0.100383875),
    entry2(-0.6995302564, 0.7146029809),
    entry2(0.5263414922, -0.85027327),
    entry2(-0.5395221479, 0.841971408),
    entry2(0.6579370318, 0.7530729462),
    entry2(0.01426758847, -0.9998982128),
    entry2(-0.6734383991, 0.7392433447),
    entry2(0.639412098, -0.7688642071),
    entry2(0.9211571421, 0.3891908523),
    entry2(-0.146637214, -0.9891903394),
    entry2(-0.782318098, 0.6228791163),
    entry2(-0.5039610839, -0.8637263605),
    entry2(-0.7743120191, -0.6328039957),
]);

pub(crate) const RAND_VECS_3D: Table3<256> = Table3::new([
    entry3(-0.7292736885, -0.6618439697, 0.1735581948),
    entry3(0.790292081, -0.5480887466, -0.2739291014),
    entry3(0.7217578935, 0.6226212466, -0.3023380997),
    entry3(0.565683137, -0.8208298145, -0.0790000257),
    entry3(0.760049034, -0.5555979497, -0.3370999617),
    entry3(0.3713945616, 0.5011264475, 0.7816254623),
    entry3(-0.1277062463, -0.4254438999, -0.8959289049),
    entry3(-0.2881560924, -0.5815838982, 0.7607405838),
    entry3(0.5849561111, -0.662820239, -0.4674352136),
    entry3(0.3307171178, 0.0391653737, 0.94291689),
    entry3(0.8712121778, -0.4113374369, -0.2679381538),
    entry3(0.580981015, 0.7021915846, 0.4115677815),
    entry3(0.503756873, 0.6330056931, -0.5878203852),
    entry3(0.4493712205, 0.601390195, 0.6606022552),
    entry3(-0.6878403724, 0.09018890807, -0.7202371714),
    entry3(-0.5958956522, -0.6469350577, 0.475797649),
    entry3(-0.5127052122, 0.1946921978, -0.8361987284),
    entry3(-0.9911507142, -0.05410276466, -0.1212153153),
    entry3(-0.2149721042, 0.9720882117, -0.09397607749),
    entry3(-0.7518650936, -0.5428057603, 0.3742469607),
    entry3(0.5237068895, 0.8516377189, -0.02107817834),
    entry3(0.6333504779, 0.1926167129, -0.7495104896),
    entry3(-0.06788241606, 0.3998305789, 0.9140719259),
    entry3(-0.5538628599, -0.4729896695, -0.6852128902),
    entry3(-0.7261455366, -0.5911990757, 0.3509933228),
    entry3(-0.9229274737, -0.1782808786, 0.3412049336),
    entry3(-0.6968815002, 0.6511274338, 0.3006480328),
    entry3(0.9608044783, -0.2098363234, -0.1811724921),
    entry3(0.06817146062, -0.9743405129, 0.2145069156),
    entry3(-0.3577285196, -0.6697087264, -0.6507845481),
    entry3(-0.1868621131, 0.7648617052, -0.6164974636),
    entry3(-0.6541697588, 0.3967914832, 0.6439087246),
    entry3(0.6993340405, -0.6164538506, 0.3618239211),
    entry3(-0.1546665739, 0.6291283928, 0.7617583057),
    entry3(-0.6841612949, -0.2580482182, -0.6821542638),
    entry3(0.5383980957, 0.4258654885, 0.7271630328),
    entry3(-0.5026987823, -0.7939832935, -0.3418836993),
    entry3(0.3202971715, 0.2834415347, 0.9039195862),
    entry3(0.8683227101, -0.0003762656404, -0.4959995258),
    entry3(0.791120031, -0.08511045745, 0.6057105799),
    entry3(-0.04011016052, -0.4397248749, 0.8972364289),
    entry3(0.9145119872, 0.3579346169, -0.1885487608),
    entry3(-0.9612039066, -0.2756484276, 0.01024666929),
    entry3(0.6510361721, -0.2877799159, -0.7023778346),
    entry3(-0.2041786351, 0.7365237271, 0.644859585),
    entry3(-0.7718263711, 0.3790626912, 0.5104855816),
    entry3(-0.3060082741, -0.7692987727, 0.5608371729),
    entry3(0.454007341, -0.5024843065, 0.7357899537),
    entry3(0.4816795475, 0.6021208291, -0.6367380315),
    entry3(0.6961980369, -0.3222197429, 0.641469197),
    entry3(-0.6532160499, -0.6781148932, 0.3368515753),
    entry3(0.5089301236, -0.6154662304, -0.6018234363),
    entry3(-0.1635919754, -0.9133604627, -0.372840892),
    entry3(0.52408019, -0.8437664109, 0.1157505864),
    entry3(0.5902587356, 0.4983817807, -0.6349883666),
    entry3(0.5863227872, 0.494764745, 0.6414307729),
    entry3(0.6779335087, 0.2341345225, 0.6968408593),
    entry3(0.7177054546, -0.6858979348, 0.120178631),
    entry3(-0.5328819713, -0.5205125012, 0.6671608058),
    entry3(-0.8654874251, -0.0700727088, -0.4960053754),
    entry3(-0.2861810166, 0.7952089234, 0.5345495242),
    entry3(-0.04849529634, 0.9810836427, -0.1874115585),
    entry3(-0.6358521667, 0.6058348682, 0.4781800233),
    entry3(0.6254794696, -0.2861619734, 0.7258696564),
    entry3(-0.2585259868, 0.5061949264, -0.8227581726),
    entry3(0.02136306781, 0.5064016808, -0.8620330371),
    entry3(0.200111773, 0.8599263484, 0.4695550591),
    entry3(0.4743561372, 0.6014985084, -0.6427953014),
    entry3(0.6622993731, -0.5202474575, -0.5391679918),
    entry3(0.08084972818, -0.6532720452, 0.7527940996),
    entry3(-0.6893687501, 0.0592860349, 0.7219805347),
    entry3(-0.1121887082, -0.9673185067, 0.2273952515),
    entry3(0.7344116094, 0.5979668656, -0.3210532909),
    entry3(0.5789393465, -0.2488849713, 0.7764570201),
    entry3(0.6988182827, 0.3557169806, -0.6205791146),
    entry3(-0.8636845529, -0.2748771249, -0.4224826141),
    entry3(-0.4247027957, -0.4640880967, 0.777335046),
    entry3(0.5257722489, -0.8427017621, 0.1158329937),
    entry3(0.9343830603, 0.316302472, -0.1639543925),
    entry3(-0.1016836419, -0.8057303073, -0.5834887393),
    entry3(-0.6529238969, 0.50602126, -0.5635892736),
    entry3(-0.2465286165, -0.9668205684, -0.06694497494),
    entry3(-0.9776897119, -0.2099250524, -0.007368825344),
    entry3(0.7736893337, 0.5734244712, 0.2694238123),
    entry3(-0.6095087895, 0.4995678998, 0.6155736747),
    entry3(0.5794535482, 0.7434546771, 0.3339292269),
    entry3(-0.8226211154, 0.08142581855, 0.5627293636),
    entry3(-0.510385483, 0.4703667658, 0.7199039967),
    entry3(-0.5764971849, -0.07231656274, -0.8138926898),
    entry3(0.7250628871, 0.3949971505, -0.5641463116),
    entry3(-0.1525424005, 0.4860840828, -0.8604958341),
    entry3(-0.5550976208, -0.4957820792, 0.667882296),
    entry3(-0.1883614327, 0.9145869398, 0.357841725),
    entry3(0.7625556724, -0.5414408243, -0.3540489801),
    entry3(-0.5870231946, -0.3226498013, -0.7424963803),
    entry3(0.3051124198, 0.2262544068, -0.9250488391),
    entry3(0.6379576059, 0.577242424, -0.5097070502),
    entry3(-0.5966775796, 0.1454852398, -0.7891830656),
    entry3(-0.658330573, 0.6555487542, -0.3699414651),
    entry3(0.7434892426, 0.2351084581, 0.6260573129),
    entry3(0.5562114096, 0.8264360377, -0.0873632843),
    entry3(-0.3028940016, -0.8251527185, 0.4768419182),
    entry3(0.1129343818, -0.985888439, -0.1235710781),
    entry3(0.5937652891, -0.5896813806, 0.5474656618),
    entry3(0.6757964092, -0.5835758614, -0.4502648413),
    entry3(0.7242302609, -0.1152719764, 0.6798550586),
    entry3(-0.9511914166, 0.0753623979, -0.2992580792),
    entry3(0.2539470961, -0.1886339355, 0.9486454084),
    entry3(0.571433621, -0.1679450851, -0.8032795685),
    entry3(-0.06778234979, 0.3978269256, 0.9149531629),
    entry3(0.6074972649, 0.733060024, -0.3058922593),
    entry3(-0.5435478392, 0.1675822484, 0.8224791405),
    entry3(-0.5876678086, -0.3380045064, -0.7351186982),
    entry3(-0.7967562402, 0.04097822706, -0.6029098428),
    entry3(-0.1996350917, 0.8706294745, 0.4496111079),
    entry3(-0.02787660336, -0.9106232682, -0.4122962022),
    entry3(-0.7797625996, -0.6257634692, 0.01975775581),
    entry3(-0.5211232846, 0.7401644346, -0.4249554471),
    entry3(0.8575424857, 0.4053272873, -0.3167501783),
    entry3(0.1045223322, 0.8390195772, -0.5339674439),
    entry3(0.3501822831, 0.9242524096, -0.1520850155),
    entry3(0.1987849858, 0.07647613266, 0.9770547224),
    entry3(0.7845996363, 0.6066256811, -0.1280964233),
    entry3(0.09006737436, -0.9750989929, -0.2026569073),
    entry3(-0.8274343547, -0.542299559, 0.1458203587),
    entry3(-0.3485797732, -0.415802277, 0.840000362),
    entry3(-0.2471778936, -0.7304819962, -0.6366310879),
    entry3(-0.3700154943, 0.8577948156, 0.3567584454),
    entry3(0.5913394901, -0.548311967, -0.5913303597),
    entry3(0.1204873514, -0.7626472379, -0.6354935001),
    entry3(0.616959265, 0.03079647928, 0.7863922953),
    entry3(0.1258156836, -0.6640829889, -0.7369967419),
    entry3(-0.6477565124, -0.1740147258, -0.7417077429),
    entry3(0.6217889313, -0.7804430448, -0.06547655076),
    entry3(0.6589943422, -0.6096987708, 0.4404473475),
    entry3(-0.2689837504, -0.6732403169, -0.6887635427),
    entry3(-0.3849775103, 0.5676542638, 0.7277093879),
    entry3(0.5754444408, 0.8110471154, -0.1051963504),
    entry3(0.9141593684, 0.3832947817, 0.131900567),
    entry3(-0.107925319, 0.9245493968, 0.3654593525),
    entry3(0.377977089, 0.3043148782, 0.8743716458),
    entry3(-0.2142885215, -0.8259286236, 0.5214617324),
    entry3(0.5802544474, 0.4148098596, -0.7008834116),
    entry3(-0.1982660881, 0.8567161266, -0.4761596756),
    entry3(-0.03381553704, 0.3773180787, -0.9254661404),
    entry3(-0.6867922841, -0.6656597827, 0.2919133642),
    entry3(0.7731742607, -0.2875793547, -0.5652430251),
    entry3(-0.09655941928, 0.9193708367, -0.3813575004),
    entry3(0.2715702457, -0.9577909544, -0.09426605581),
    entry3(0.2451015704, -0.6917998565, -0.6792188003),
    entry3(0.977700782, -0.1753855374, 0.1155036542),
    entry3(-0.5224739938, 0.8521606816, 0.02903615945),
    entry3(-0.7734880599, -0.5261292347, 0.3534179531),
    entry3(-0.7134492443, -0.269547243, 0.6467878011),
    entry3(0.1644037271, 0.5105846203, -0.8439637196),
    entry3(0.6494635788, 0.05585611296, 0.7583384168),
    entry3(-0.4711970882, 0.5017280509, -0.7254255765),
    entry3(-0.6335764307, -0.2381686273, -0.7361091029),
    entry3(-0.9021533097, -0.270947803, -0.3357181763),
    entry3(-0.3793711033, 0.872258117, 0.3086152025),
    entry3(-0.6855598966, -0.3250143309, 0.6514394162),
    entry3(0.2900942212, -0.7799057743, -0.5546100667),
    entry3(-0.2098319339, 0.85037073, 0.4825351604),
    entry3(-0.4592603758, 0.6598504336, -0.5947077538),
    entry3(0.8715945488, 0.09616365406, -0.4807031248),
    entry3(-0.6776666319, 0.7118504878, -0.1844907016),
    entry3(0.7044377633, 0.312427597, 0.637304036),
    entry3(-0.7052318886, -0.2401093292, -0.6670798253),
    entry3(0.081921007, -0.7207336136, -0.6883545647),
    entry3(-0.6993680906, -0.5875763221, -0.4069869034),
    entry3(-0.1281454481, 0.6419895885, 0.7559286424),
    entry3(-0.6337388239, -0.6785471501, -0.3714146849),
    entry3(0.5565051903, -0.2168887573, -0.8020356851),
    entry3(-0.5791554484, 0.7244372011, -0.3738578718),
    entry3(0.1175779076, -0.7096451073, 0.6946792478),
    entry3(-0.6134619607, 0.1323631078, 0.7785527795),
    entry3(0.6984635305, -0.02980516237, -0.715024719),
    entry3(0.8318082963, -0.3930171956, 0.3919597455),
    entry3(0.1469576422, 0.05541651717, -0.9875892167),
    entry3(0.708868575, -0.2690503865, 0.6520101478),
    entry3(0.2726053183, 0.67369766, -0.68688995),
    entry3(-0.6591295371, 0.3035458599, -0.6880466294),
    entry3(0.4815131379, -0.7528270071, 0.4487723203),
    entry3(0.9430009463, 0.1675647412, -0.2875261255),
    entry3(0.434802957, 0.7695304522, -0.4677277752),
    entry3(0.3931996188, 0.594473625, 0.7014236729),
    entry3(0.7254336655, -0.603925654, 0.3301814672),
    entry3(0.7590235227, -0.6506083235, 0.02433313207),
    entry3(-0.8552768592, -0.3430042733, 0.3883935666),
    entry3(-0.6139746835, 0.6981725247, 0.3682257648),
    entry3(-0.7465905486, -0.5752009504, 0.3342849376),
    entry3(0.5730065677, 0.810555537, -0.1210916791),
    entry3(-0.9225877367, -0.3475211012, -0.167514036),
    entry3(-0.7105816789, -0.4719692027, -0.5218416899),
    entry3(-0.08564609717, 0.3583001386, 0.929669703),
    entry3(-0.8279697606, -0.2043157126, 0.5222271202),
    entry3(0.427944023, 0.278165994, 0.8599346446),
    entry3(0.5399079671, -0.7857120652, -0.3019204161),
    entry3(0.5678404253, -0.5495413974, -0.6128307303),
    entry3(-0.9896071041, 0.1365639107, -0.04503418428),
    entry3(-0.6154342638, -0.6440875597, 0.4543037336),
    entry3(0.1074204368, -0.7946340692, 0.5975094525),
    entry3(-0.3595449969, -0.8885529948, 0.28495784),
    entry3(-0.2180405296, 0.1529888965, 0.9638738118),
    entry3(-0.7277432317, -0.6164050508, -0.3007234646),
    entry3(0.7249729114, -0.00669719484, 0.6887448187),
    entry3(-0.5553659455, -0.5336586252, 0.6377908264),
    entry3(0.5137558015, 0.7976208196, -0.3160000073),
    entry3(-0.3794024848, 0.9245608561, -0.03522751494),
    entry3(0.8229248658, 0.2745365933, -0.4974176556),
    entry3(-0.5404114394, 0.6091141441, 0.5804613989),
    entry3(0.8036581901, -0.2703029469, 0.5301601931),
    entry3(0.6044318879, 0.6832968393, 0.4095943388),
    entry3(0.06389988817, 0.9658208605, -0.2512108074),
    entry3(0.1087113286, 0.7402471173, -0.6634877936),
    entry3(-0.713427712, -0.6926784018, 0.1059128479),
    entry3(0.6458897819, -0.5724548511, -0.5050958653),
    entry3(-0.6553931414, 0.7381471625, 0.159995615),
    entry3(0.3910961323, 0.9188871375, -0.05186755998),
    entry3(-0.4879022471, -0.5904376907, 0.6429111375),
    entry3(0.6014790094, 0.7707441366, -0.2101820095),
    entry3(-0.5677173047, 0.7511360995, 0.3368851762),
    entry3(0.7858573506, 0.226674665, 0.5753666838),
    entry3(-0.4520345543, -0.604222686, -0.6561857263),
    entry3(0.002272116345, 0.4132844051, -0.9105991643),
    entry3(-0.5815751419, -0.5162925989, 0.6286591339),
    entry3(-0.03703704785, 0.8273785755, 0.5604221175),
    entry3(-0.5119692504, 0.7953543429, -0.3244980058),
    entry3(-0.2682417366, -0.9572290247, -0.1084387619),
    entry3(-0.2322482736, -0.9679131102, -0.09594243324),
    entry3(0.3554328906, -0.8881505545, 0.2913006227),
    entry3(0.7346520519, -0.4371373164, 0.5188422971),
    entry3(0.9985120116, 0.04659011161, -0.02833944577),
    entry3(-0.3727687496, -0.9082481361, 0.1900757285),
    entry3(0.91737377, -0.3483642108, 0.1925298489),
    entry3(0.2714911074, 0.4147529736, -0.8684886582),
    entry3(0.5131763485, -0.7116334161, 0.4798207128),
    entry3(-0.8737353606, 0.18886992, -0.4482350644),
    entry3(0.8460043821, -0.3725217914, 0.3814499973),
    entry3(0.8978727456, -0.1780209141, -0.4026575304),
    entry3(0.2178065647, -0.9698322841, -0.1094789531),
    entry3(-0.1518031304, -0.7788918132, -0.6085091231),
    entry3(-0.2600384876, -0.4755398075, -0.8403819825),
    entry3(0.572313509, -0.7474340931, -0.3373418503),
    entry3(-0.7174141009, 0.1699017182, -0.6756111411),
    entry3(-0.684180784, 0.02145707593, -0.7289967412),
    entry3(-0.2007447902, 0.06555605789, -0.9774476623),
    entry3(-0.1148803697, -0.8044887315, 0.5827524187),
    entry3(-0.7870349638, 0.03447489231, 0.6159443543),
    entry3(-0.2015596421, 0.6859872284, 0.6991389226),
    entry3(-0.08581082512, -0.10920836, -0.9903080513),
    entry3(0.5532693395, 0.7325250401, -0.396610771),
    entry3(-0.1842489331, -0.9777375055, -0.1004076743),
    entry3(0.0775473789, -0.9111505856, 0.4047110257),
    entry3(0.1399838409, 0.7601631212, -0.6344734459),
    entry3(0.4484419361, -0.845289248, 0.2904925424),
]);

pub(crate) const GRADIENTS_2D: Table2<128> = Table2::new([
    entry2(0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.608761429008721, 0.793353340291235),
    entry2(0.793353340291235, 0.608761429008721),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.99144486137381, 0.130526192220051),
    entry2(0.99144486137381, -0.130526192220051),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.793353340291235, -0.60876142900872),
    entry2(0.608761429008721, -0.793353340291235),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(0.130526192220052, -0.99144486137381),
    entry2(-0.130526192220052, -0.99144486137381),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.608761429008721, -0.793353340291235),
    entry2(-0.793353340291235, -0.608761429008721),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.99144486137381, -0.130526192220052),
    entry2(-0.99144486137381, 0.130526192220051),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.793353340291235, 0.608761429008721),
    entry2(-0.608761429008721, 0.793353340291235),
    entry2(-0.38268343236509, 0.923879532511287),
    entry2(-0.130526192220052, 0.99144486137381),
    entry2(0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.608761429008721, 0.793353340291235),
    entry2(0.793353340291235, 0.608761429008721),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.99144486137381, 0.130526192220051),
    entry2(0.99144486137381, -0.130526192220051),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.793353340291235, -0.60876142900872),
    entry2(0.608761429008721, -0.793353340291235),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(0.130526192220052, -0.99144486137381),
    entry2(-0.130526192220052, -0.99144486137381),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.608761429008721, -0.793353340291235),
    entry2(-0.793353340291235, -0.608761429008721),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.99144486137381, -0.130526192220052),
    entry2(-0.99144486137381, 0.130526192220051),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.793353340291235, 0.608761429008721),
    entry2(-0.608761429008721, 0.793353340291235),
    entry2(-0.38268343236509, 0.923879532511287),
    entry2(-0.130526192220052, 0.99144486137381),
    entry2(0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.608761429008721, 0.793353340291235),
    entry2(0.793353340291235, 0.608761429008721),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.99144486137381, 0.130526192220051),
    entry2(0.99144486137381, -0.130526192220051),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.793353340291235, -0.60876142900872),
    entry2(0.608761429008721, -0.793353340291235),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(0.130526192220052, -0.99144486137381),
    entry2(-0.130526192220052, -0.99144486137381),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.608761429008721, -0.793353340291235),
    entry2(-0.793353340291235, -0.608761429008721),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.99144486137381, -0.130526192220052),
    entry2(-0.99144486137381, 0.130526192220051),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.793353340291235, 0.608761429008721),
    entry2(-0.608761429008721, 0.793353340291235),
    entry2(-0.38268343236509, 0.923879532511287),
    entry2(-0.130526192220052, 0.99144486137381),
    entry2(0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.608761429008721, 0.793353340291235),
    entry2(0.793353340291235, 0.608761429008721),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.99144486137381, 0.130526192220051),
    entry2(0.99144486137381, -0.130526192220051),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.793353340291235, -0.60876142900872),
    entry2(0.608761429008721, -0.793353340291235),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(0.130526192220052, -0.99144486137381),
    entry2(-0.130526192220052, -0.99144486137381),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.608761429008721, -0.793353340291235),
    entry2(-0.793353340291235, -0.608761429008721),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.99144486137381, -0.130526192220052),
    entry2(-0.99144486137381, 0.130526192220051),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.793353340291235, 0.608761429008721),
    entry2(-0.608761429008721, 0.793353340291235),
    entry2(-0.38268343236509, 0.923879532511287),
    entry2(-0.130526192220052, 0.99144486137381),
    entry2(0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.608761429008721, 0.793353340291235),
    entry2(0.793353340291235, 0.608761429008721),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.99144486137381, 0.130526192220051),
    entry2(0.99144486137381, -0.130526192220051),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.793353340291235, -0.60876142900872),
    entry2(0.608761429008721, -0.793353340291235),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(0.130526192220052, -0.99144486137381),
    entry2(-0.130526192220052, -0.99144486137381),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.608761429008721, -0.793353340291235),
    entry2(-0.793353340291235, -0.608761429008721),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.99144486137381, -0.130526192220052),
    entry2(-0.99144486137381, 0.130526192220051),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.793353340291235, 0.608761429008721),
    entry2(-0.608761429008721, 0.793353340291235),
    entry2(-0.38268343236509, 0.923879532511287),
    entry2(-0.130526192220052, 0.99144486137381),
    entry2(0.38268343236509, 0.923879532511287),
    entry2(0.923879532511287, 0.38268343236509),
    entry2(0.923879532511287, -0.38268343236509),
    entry2(0.38268343236509, -0.923879532511287),
    entry2(-0.38268343236509, -0.923879532511287),
    entry2(-0.923879532511287, -0.38268343236509),
    entry2(-0.923879532511287, 0.38268343236509),
    entry2(-0.38268343236509, 0.923879532511287),
]);

pub(crate) const GRADIENTS_3D: Table3<64> = Table3::new([
    entry3(0.0, 1.0, 1.0),
    entry3(0.0, -1.0, 1.0),
    entry3(0.0, 1.0, -1.0),
    entry3(0.0, -1.0, -1.0),
    entry3(1.0, 0.0, 1.0),
    entry3(-1.0, 0.0, 1.0),
    entry3(1.0, 0.0, -1.0),
    entry3(-1.0, 0.0, -1.0),
    entry3(1.0, 1.0, 0.0),
    entry3(-1.0, 1.0, 0.0),
    entry3(1.0, -1.0, 0.0),
    entry3(-1.0, -1.0, 0.0),
    entry3(0.0, 1.0, 1.0),
    entry3(0.0, -1.0, 1.0),
    entry3(0.0, 1.0, -1.0),
    entry3(0.0, -1.0, -1.0),
    entry3(1.0, 0.0, 1.0),
    entry3(-1.0, 0.0, 1.0),
    entry3(1.0, 0.0, -1.0),
    entry3(-1.0, 0.0, -1.0),
    entry3(1.0, 1.0, 0.0),
    entry3(-1.0, 1.0, 0.0),
    entry3(1.0, -1.0, 0.0),
    entry3(-1.0, -1.0, 0.0),
    entry3(0.0, 1.0, 1.0),
    entry3(0.0, -1.0, 1.0),
    entry3(0.0, 1.0, -1.0),
    entry3(0.0, -1.0, -1.0),
    entry3(1.0, 0.0, 1.0),
    entry3(-1.0, 0.0, 1.0),
    entry3(1.0, 0.0, -1.0),
    entry3(-1.0, 0.0, -1.0),
    entry3(1.0, 1.0, 0.0),
    entry3(-1.0, 1.0, 0.0),
    entry3(1.0, -1.0, 0.0),
    entry3(-1.0, -1.0, 0.0),
    entry3(0.0, 1.0, 1.0),
    entry3(0.0, -1.0, 1.0),
    entry3(0.0, 1.0, -1.0),
    entry3(0.0, -1.0, -1.0),
    entry3(1.0, 0.0, 1.0),
    entry3(-1.0, 0.0, 1.0),
    entry3(1.0, 0.0, -1.0),
    entry3(-1.0, 0.0, -1.0),
    entry3(1.0, 1.0, 0.0),
    entry3(1.0, 1.0, 0.0),
    entry3(1.0, -1.0, 0.0),
    entry3(-1.0, -1.0, 0.0),
    entry3(0.0, 1.0, 1.0),
    entry3(0.0, -1.0, 1.0),
    entry3(0.0, 1.0, -1.0),
    entry3(0.0, -1.0, -1.0),
    entry3(1.0, 0.0, 1.0),
    entry3(-1.0, 0.0, 1.0),
    entry3(1.0, 0.0, -1.0),
    entry3(-1.0, 0.0, -1.0),
    entry3(1.0, 1.0, 0.0),
    entry3(-1.0, 1.0, 0.0),
    entry3(1.0, -1.0, 0.0),
    entry3(-1.0, -1.0, 0.0),
    entry3(1.0, 1.0, 0.0),
    entry3(0.0, -1.0, 1.0),
    entry3(-1.0, 1.0, 0.0),
    entry3(0.0, -1.0, -1.0),
]);
