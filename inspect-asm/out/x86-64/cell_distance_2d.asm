inspect_asm::cell_distance_2d:
	push rbp
	push r15
	push r14
	push rbx
	movd xmm0, edi
	xorps xmm1, xmm1
	cmpnless xmm1, xmm0
	movss xmm3, dword ptr [rip + .LCPI5_0]
	movaps xmm2, xmm1
	andps xmm2, xmm3
	movss xmm4, dword ptr [rip + .LCPI5_1]
	andnps xmm1, xmm4
	orps xmm1, xmm2
	addss xmm1, xmm0
	cvttss2si eax, xmm1
	shr rdi, 32
	movss xmm5, dword ptr [rip + .LCPI5_2]
	ucomiss xmm1, xmm5
	mov ecx, 2147483647
	cmova eax, ecx
	movd xmm2, edi
	xor esi, esi
	ucomiss xmm1, xmm1
	xorps xmm1, xmm1
	cmovp eax, esi
	cmpnless xmm1, xmm2
	andps xmm3, xmm1
	andnps xmm1, xmm4
	orps xmm1, xmm3
	addss xmm1, xmm2
	cvttss2si edx, xmm1
	ucomiss xmm1, xmm5
	cmova edx, ecx
	ucomiss xmm1, xmm1
	cmovp edx, esi
	lea ecx, [rax - 1]
	inc eax
	movss xmm1, dword ptr [rip + .LCPI5_3]
	cmp ecx, eax
	jle .LBB_1
.LBB_10:
	xorps xmm0, xmm0
	sqrtss xmm0, xmm1
	addss xmm0, dword ptr [rip + .LCPI5_5]
	pop rbx
	pop r14
	pop r15
	pop rbp
	ret
.LBB_1:
	lea esi, [rdx - 1]
	inc edx
	cmp esi, edx
	jle .LBB_2
.LBB_8:
	xor edx, edx
	cmp ecx, eax
	setl sil
	jge .LBB_10
	mov dl, sil
	add ecx, edx
	cmp ecx, eax
	jle .LBB_8
	jmp .LBB_10
.LBB_2:
	imul edi, esi, 1136930381
	imul r8d, ecx, 501125321
	movss xmm1, dword ptr [rip + .LCPI5_3]
	lea r9, [rip + .L__unnamed__0]
	movaps xmm3, xmmword ptr [rip + .LCPI5_4]
.LBB_3:
	xor r10d, r10d
	cmp ecx, eax
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	setl r10b
	subss xmm4, xmm0
	mov r11d, edi
	mov ebx, esi
.LBB_4:
	xor ebp, ebp
	cmp ebx, edx
	setl r14b
	mov r15d, r11d
	xor r15d, r8d
	imul r15d, r15d, 301
	and r15d, 510
	xorps xmm5, xmm5
	cvtsi2ss xmm5, ebx
	subss xmm5, xmm2
	movsd xmm6, qword ptr [r9 + 4*r15]
	mulps xmm6, xmm3
	movlhps xmm5, xmm4
	shufps xmm5, xmm4, 226
	addps xmm5, xmm6
	mulps xmm5, xmm5
	movaps xmm6, xmm5
	shufps xmm6, xmm5, 85
	addss xmm6, xmm5
	minss xmm1, xmm6
	cmp ebx, edx
	jge .LBB_6
	mov bpl, r14b
	add ebx, ebp
	add r11d, 1136930381
	cmp ebx, edx
	jle .LBB_4
.LBB_6:
	cmp ecx, eax
	jge .LBB_10
	add ecx, r10d
	add r8d, 501125321
	cmp ecx, eax
	jle .LBB_3
	jmp .LBB_10