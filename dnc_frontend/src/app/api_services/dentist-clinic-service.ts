// src/app/api_services/dentist-clinic-service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {map, Observable} from 'rxjs';

import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

//
// ---- Types that mirror your Rust struct(s)
//

export interface DentistClinicWithNames {
    dentist_id: number;
    clinic_id: number | null;

    position_id: number | null;
    schedule: string | null;

    last_name: string;
    given_name: string;
    middle_name: string | null;

    clinic_name: string | null;
    position_name: string | null;
}

export interface DentistClinicWithNamesAndAddress {
    dentist_id: number;
    clinic_id: number | null;

    position_id: number | null;
    schedule: string | null;

    last_name: string;
    given_name: string;
    middle_name: string | null;

    clinic_name: string | null;
    clinic_address: string | null;
    position_name: string | null;
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

    getClinicsForDentistId(dentistId: number): Observable<DentistClinicWithNamesAndAddress[]> {
        return this.http.get<DentistClinicWithNamesAndAddress[]>(
            `${this.baseUrl}/dentists/${encodeURIComponent(String(dentistId))}/clinics`, { headers: this.authHeaders()})
            .pipe(map(rows => rows.map(this.addAddressToClinicName)));
    }

    addAddressToClinicName(row: DentistClinicWithNamesAndAddress):DentistClinicWithNamesAndAddress{
        row.clinic_name = row.clinic_name + " (" + row.clinic_address + ")";
        return row;
    }

    //
    // ---- GET /clinics/{clinic_id}/dentists
    //

    getDentistsForClinicId(clinicId: number): Observable<DentistClinicWithNames[]> {
        return this.http.get<DentistClinicWithNames[]>(
            `${this.baseUrl}/dental_clinics/${encodeURIComponent(String(clinicId))}/dentists`, { headers: this.authHeaders()}
        );
    }
    addDentistClinic(clinicId: number, dentistId: number, position_id:number|null, schedule:string|null): Observable<DentistClinicWithNames[]> {
        const payload = { clinic_id: clinicId, position_id: position_id, schedule: schedule };
        return this.http.post<DentistClinicWithNames[]>(
            `${this.baseUrl}/dentists/${encodeURIComponent(String(dentistId))}/clinics`, payload, { headers: this.authHeaders()}
        );
    }

    removeDentistClinic(clinicId: number, dentistId: number): Observable<DentistClinicWithNames[]> {
        return this.http.delete<DentistClinicWithNames[]>(
            `${this.baseUrl}/dentists/${encodeURIComponent(String(dentistId))}/clinics/${encodeURIComponent(String(clinicId))}`, { headers: this.authHeaders()}
        );
    }
}
