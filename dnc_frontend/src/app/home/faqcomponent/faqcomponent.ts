import { Component } from '@angular/core';
import { NgOptimizedImage } from '@angular/common';
import { MatIconModule } from '@angular/material/icon';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

interface FaqItem {
    question: string;
    answer: string;
    bullets?: string[];
}

@Component({
    selector: 'app-faq',
    standalone: true,
    imports: [
        NgOptimizedImage,
        MatIconModule,
        MatExpansionModule,
        MatCardModule,
        MatButtonModule,
    ],
    templateUrl: './faqcomponent.html',
    styleUrl: './faqcomponent.scss',
})
export class FAQComponent {

    readonly faqs: FaqItem[] = [
        {
            question: 'Who can use the dental benefits?',
            answer:
                'Dental benefits are available to employees and enrolled dependents (if applicable) covered under their company\'s dental plan.',
        },
        {
            question: 'How can I check if my dental benefits are active?',
            answer:
                'You may contact DNC or your company\'s HR Benefits Office or HMO provider to verify your eligibility and benefit coverage.',
        },
        {
            question: 'How do I avail of my dental benefits?',
            answer:
                'Schedule an appointment with a DNC-accredited dental clinic. The list of nationwide accredited dental clinics are available at dnc.com.ph.',
            bullets: [
                'Provide the accredited clinic your company name and HMO or dental card number (if applicable).',
                'Wait for your appointment to be confirmed by the dental clinic.',
            ],
        },
        {
            question: 'What dental services are covered?',
            answer:
                'Coverage varies depending on your company\'s dental plan. For details, please contact either Dental Network Company, your HR Benefits Office or HMO Account / Liaison Office.',
        },
        {
            question: 'Are there any limitations or exclusions?',
            answer:
                'Yes. After your dental examination, the dentist will explain your treatment plan, including any services that are not covered by your dental benefits.',
        },
        {
            question: 'What if I need a treatment that\'s not covered?',
            answer:
                'You may still proceed with the treatment by paying for the non-covered services at specially discounted rates offered by DNC-accredited clinics.',
        },
        {
            question: 'Can I transfer my dental benefits to someone else?',
            answer:
                'Dental benefits are strictly non-transferable and may only be used by the enrolled member and eligible dependents.',
        },
        {
            question: 'Can I visit any dentist or dental clinic?',
            answer:
                'Dental benefits may only be used at DNC-accredited dental clinics.',
        },
        {
            question: 'How can I find the nearest DNC-accredited dental clinic?',
            answer:
                'Visit dnc.com.ph or call the DNC Hotline for assistance in locating the nearest accredited clinic.',
            bullets: [
                '(02) 8911-7777',
                '0916-7615277 (Globe / Viber)',
                '0945-5387714 (Globe / Viber)',
                '0923-8095376 (Sun / Viber)',
            ],
        },
        {
            question: 'What should I bring to my appointment?',
            answer:
                'Please bring any of the following for verification:',
            bullets: [
                'HMO or dental card (if applicable)',
                'Company ID',
                'Valid government-issued ID',
            ],
        },
        {
            question: 'Will I need to pay during my dental visit?',
            answer:
                'Covered dental services are cashless. If your treatment includes services that are not covered by your dental benefits, you will only need to pay for those services. To avoid any misunderstanding, we recommend asking your dentist about any applicable charges before proceeding with treatment.',
        },
        {
            question: 'Can my dependents also use the dental benefits?',
            answer:
                'Yes, as long as they are enrolled and eligible under your company\'s dental plan.',
        },
        {
            question: 'Can I reschedule or cancel my appointment?',
            answer:
                'Yes. Please notify your DNC-accredited dental clinic as early as possible, preferably at least one (1) day before your scheduled appointment.',
        },
        {
            question: 'What should I do if my dental benefit request is denied?',
            answer:
                'Please contact DNC so we can review and assist with your concern. You may reach us through email at concerns@dnc.com.ph or call via hotline.',
            bullets: [
                '(02) 8911-7777',
                '0916-7615277 (Globe / Viber)',
                '0945-5387714 (Globe / Viber)',
                '0923-8095376 (Sun / Viber)',
            ],
        },
        {
            question: 'Who can I contact if I have questions?',
            answer:
                'For any questions about your dental benefits, you may reach us through email at concerns@dnc.com.ph or call via hotline.',
            bullets: [
                '(02) 8911-7777',
                '0916-7615277 (Globe / Viber)',
                '0945-5387714 (Globe / Viber)',
                '0923-8095376 (Sun / Viber)',
            ],
        },
    ];
}
