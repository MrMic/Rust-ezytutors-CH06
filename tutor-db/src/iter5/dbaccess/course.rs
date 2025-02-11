use sqlx::{postgres::PgPool, types::chrono};

use crate::errors::EzyTutorError;
use crate::models::course::Course;

// ______________________________________________________________________
pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorError> {
    // * INFO: Prepare SQL Statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool)
    .await?;

    // * INFO: Extract result
    let courses: Vec<crate::models::Course> = course_rows
        .iter()
        .map(|course_row| Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
        .collect();
    match courses.len() {
        0 => Err(EzyTutorError::NotFound(
            "Courses not found for tutor".into(),
        )),
        _ => Ok(courses),
    }
}

// ______________________________________________________________________
pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorError> {
    // * INFO: Prepare SQL Statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course_row) = course_row {
        // * INFO: Execute query
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(course_row.posted_time.unwrap()),
        })
    } else {
        Err(EzyTutorError::NotFound(
            "Course id not found for tutor".into(),
        ))
    }
}

// ______________________________________________________________________
pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: Course,
) -> Result<Course, EzyTutorError> {
    // * INFO: Prepare SQL Statement
    let course_row = sqlx::query!(
        "INSERT INTO ezy_course_c5 (course_id,tutor_id, course_name) VALUES ($1, $2, $3) RETURNING tutor_id, course_id, course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await?;

    // * INFO: Retrieve result
    Ok(Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(course_row.posted_time.unwrap()),
    })
}
