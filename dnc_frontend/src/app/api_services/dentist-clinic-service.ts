// src/app/api_services/dentist-clinic-service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';

import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

//
// ---- Types that mirror your Rust struct(s)
//

export interface DentistClinicWithNames {
    dentist_id: number;
    clinic_id: number | null;

    position: string | null;
    schedule: string | null;

    last_name: string;
    given_name: string;
    middle_name: string | null;

    clinic_name: string | null;
}

@Injectable({ providedIn: 'root' })
export class DentistClinicService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    /**
     * Base API URL.
     * Adjust if your backend uses a different prefix (e.g. /api).
     */
    private readonly baseUrl = `${environment.apiUrl}/api`;
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    //
    // ---- GET /dentist_clinics
    //

    getAllDentistClinics(): Observable<DentistClinicWithNames[]> {
        return this.http.get<DentistClinicWithNames[]>(
            `${this.baseUrl}`
        );
    }

    //
    // ---- GET /dentists/{dentist_id}/clinics
    //

    getClinicsForDentistId(dentistId: number): Observable<DentistClinicWithNames[]> {
        return this.http.get<DentistClinicWithNames[]>(
            `${this.baseUrl}/dentists/${encodeURIComponent(String(dentistId))}/clinics`, { headers: this.authHeaders()}
        );
    }

    //
    // ---- GET /clinics/{clinic_id}/dentists
    //

    getDentistsForClinicId(clinicId: number): Observable<DentistClinicWithNames[]> {
        return this.http.get<DentistClinicWithNames[]>(
            `${this.baseUrl}/dental_clinics/${encodeURIComponent(String(clinicId))}/dentists`, { headers: this.authHeaders()}
        );
    }
}
