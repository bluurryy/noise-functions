inspect_asm::open_simplex2s_3d:
	sub rsp, 24
	movss xmm0, dword ptr [rdi + 8]
	movsd xmm1, qword ptr [rdi]
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 85
	addss xmm2, xmm1
	addss xmm2, xmm0
	mulss xmm2, dword ptr [rip + .LCPI18_0]
	movaps xmm3, xmm2
	unpcklps xmm3, xmm2
	subps xmm3, xmm1
	subss xmm2, xmm0
	movlps qword ptr [rsp + 8], xmm3
	movss dword ptr [rsp + 16], xmm2
	lea rdi, [rsp + 8]
	call noise_functions::scalar::open_simplex_2s::gen3
	add rsp, 24
	ret