inspect_asm::cell_value_2d_simd:
	push rbp
	push r15
	push r14
	push r12
	push rbx
	movsd xmm1, qword ptr [rdi]
	xorps xmm0, xmm0
	cmpleps xmm0, xmm1
	movaps xmm2, xmm0
	andnps xmm2, xmmword ptr [rip + .LCPI11_0]
	andps xmm0, xmmword ptr [rip + .LCPI11_1]
	orps xmm0, xmm2
	addps xmm0, xmm1
	cvttps2dq xmm2, xmm0
	movd eax, xmm2
	lea ecx, [rax - 1]
	inc eax
	xorps xmm0, xmm0
	cmp ecx, eax
	jg .LBB_11
	pshufd xmm2, xmm2, 85
	movd edi, xmm2
	lea edx, [rdi - 1]
	lea esi, [rdi + 1]
	cmp edx, esi
	jle .LBB_2
.LBB_9:
	xor edx, edx
	cmp ecx, eax
	setl sil
	jge .LBB_11
	mov dl, sil
	add ecx, edx
	cmp ecx, eax
	jle .LBB_9
	jmp .LBB_11
.LBB_2:
	imul edi, edi, 1136930381
	add edi, -1136930381
	imul r9d, ecx, 501125321
	xor r8d, r8d
	movss xmm2, dword ptr [rip + .LCPI11_2]
	lea r10, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI11_3]
.LBB_3:
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ecx
	mov r11d, edi
	mov ebx, edx
.LBB_4:
	movaps xmm4, xmm2
	xor ebp, ebp
	cmp ebx, esi
	setl r14b
	mov r15d, r11d
	xor r15d, r9d
	imul r15d, r15d, 668265261
	mov r12d, r15d
	and r12d, 510
	xorps xmm2, xmm2
	cvtsi2ss xmm2, ebx
	movsd xmm5, qword ptr [r10 + 4*r12]
	movlhps xmm2, xmm3
	shufps xmm2, xmm3, 226
	subps xmm2, xmm1
	mulps xmm5, xmm0
	addps xmm5, xmm2
	mulps xmm5, xmm5
	movaps xmm2, xmm5
	shufps xmm2, xmm5, 85
	addss xmm2, xmm5
	ucomiss xmm4, xmm2
	cmova r8d, r15d
	minss xmm2, xmm4
	cmp ebx, esi
	jge .LBB_6
	mov bpl, r14b
	add ebx, ebp
	add r11d, 1136930381
	cmp ebx, esi
	jle .LBB_4
.LBB_6:
	xor r11d, r11d
	cmp ecx, eax
	setl bl
	jge .LBB_8
	mov r11b, bl
	add ecx, r11d
	add r9d, 501125321
	cmp ecx, eax
	jle .LBB_3
.LBB_8:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r8d
	mulss xmm0, dword ptr [rip + .LCPI11_4]
.LBB_11:
	pop rbx
	pop r12
	pop r14
	pop r15
	pop rbp
	ret