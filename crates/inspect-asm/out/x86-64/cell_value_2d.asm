inspect_asm::cell_value_2d:
	push rbp
	push r15
	push r14
	push r12
	push rbx
	movd xmm1, edi
	xorps xmm0, xmm0
	cmpnless xmm0, xmm1
	movss xmm3, dword ptr [rip + .LCPI9_0]
	movaps xmm2, xmm0
	andps xmm2, xmm3
	movss xmm4, dword ptr [rip + .LCPI9_1]
	andnps xmm0, xmm4
	orps xmm0, xmm2
	addss xmm0, xmm1
	cvttss2si eax, xmm0
	shr rdi, 32
	movss xmm5, dword ptr [rip + .LCPI9_2]
	ucomiss xmm0, xmm5
	mov ecx, 2147483647
	cmova eax, ecx
	movd xmm2, edi
	xor esi, esi
	ucomiss xmm0, xmm0
	cmovp eax, esi
	xorps xmm0, xmm0
	cmpnless xmm0, xmm2
	andps xmm3, xmm0
	andnps xmm0, xmm4
	orps xmm0, xmm3
	addss xmm0, xmm2
	cvttss2si edx, xmm0
	ucomiss xmm0, xmm5
	cmova edx, ecx
	ucomiss xmm0, xmm0
	cmovp edx, esi
	xorps xmm0, xmm0
	lea ecx, [rax - 1]
	inc eax
	cmp ecx, eax
	jg .LBB_11
	lea esi, [rdx - 1]
	inc edx
	cmp esi, edx
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
	imul edi, esi, 1136930381
	imul r8d, ecx, 501125321
	movss xmm3, dword ptr [rip + .LCPI9_3]
	xor r9d, r9d
	lea r10, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI9_4]
.LBB_3:
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	subss xmm4, xmm1
	mov r11d, edi
	mov ebx, esi
.LBB_4:
	movaps xmm5, xmm3
	xor ebp, ebp
	cmp ebx, edx
	setl r14b
	mov r15d, r11d
	xor r15d, r8d
	imul r15d, r15d, 668265261
	mov r12d, r15d
	and r12d, 510
	xorps xmm6, xmm6
	cvtsi2ss xmm6, ebx
	subss xmm6, xmm2
	movsd xmm3, qword ptr [r10 + 4*r12]
	mulps xmm3, xmm0
	movlhps xmm6, xmm4
	shufps xmm6, xmm4, 226
	addps xmm6, xmm3
	mulps xmm6, xmm6
	movaps xmm3, xmm6
	shufps xmm3, xmm6, 85
	addss xmm3, xmm6
	ucomiss xmm5, xmm3
	cmova r9d, r15d
	minss xmm3, xmm5
	cmp ebx, edx
	jge .LBB_6
	mov bpl, r14b
	add ebx, ebp
	add r11d, 1136930381
	cmp ebx, edx
	jle .LBB_4
.LBB_6:
	xor r11d, r11d
	cmp ecx, eax
	setl bl
	jge .LBB_8
	mov r11b, bl
	add ecx, r11d
	add r8d, 501125321
	cmp ecx, eax
	jle .LBB_3
.LBB_8:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r9d
	mulss xmm0, dword ptr [rip + .LCPI9_5]
.LBB_11:
	pop rbx
	pop r12
	pop r14
	pop r15
	pop rbp
	ret