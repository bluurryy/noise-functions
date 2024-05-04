inspect_asm::open_simplex2s_3d_simd:
	sub rsp, 24
	movaps xmm0, xmmword ptr [rdi]
	movaps xmm1, xmm0
	shufps xmm1, xmm0, 85
	addss xmm1, xmm0
	movaps xmm2, xmm0
	unpckhpd xmm2, xmm0
	addss xmm2, xmm1
	mulss xmm2, dword ptr [rip + .LCPI20_0]
	shufps xmm2, xmm2, 0
	subps xmm2, xmm0
	movlps qword ptr [rsp + 8], xmm2
	movhlps xmm2, xmm2
	movss dword ptr [rsp + 16], xmm2
	lea rdi, [rsp + 8]
	call noise_functions::scalar::open_simplex_2s::gen3
	add rsp, 24
	ret